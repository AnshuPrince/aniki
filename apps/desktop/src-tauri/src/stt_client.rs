use std::sync::Arc;

use aniki_audio_core::speechmatics::{
    map_speaker, AudioFormat, EndOfStream, StartRecognition, SttServerMessage,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct SttSessionHandle {
    jwt_tx: mpsc::Sender<String>,
    shutdown: tokio::sync::watch::Sender<bool>,
}

impl Clone for SttSessionHandle {
    fn clone(&self) -> Self {
        Self {
            jwt_tx: self.jwt_tx.clone(),
            shutdown: self.shutdown.clone(),
        }
    }
}

impl SttSessionHandle {
    pub async fn update_jwt(&self, jwt: String) {
        let _ = self.jwt_tx.send(jwt).await;
    }

    pub fn stop(&self) {
        let _ = self.shutdown.send(true);
    }
}

pub async fn start_stt_session(
    app: AppHandle,
    endpoint: String,
    jwt: String,
    transcription_config: serde_json::Value,
    capture: aniki_audio_core::DualCapture,
) -> Result<SttSessionHandle, String> {
    let dev_mode = jwt.starts_with("dev-stt-jwt");
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    let (jwt_tx, jwt_rx) = mpsc::channel::<String>(4);
    let _ = jwt_tx.send(jwt).await;

    let capture = Arc::new(Mutex::new(capture));

    if dev_mode {
        let app_demo = app.clone();
        let app_mic = app.clone();
        let capture_mic = capture.clone();
        let shutdown_demo = shutdown_tx.subscribe();
        let shutdown_mic = shutdown_tx.subscribe();
        tokio::spawn(run_dev_transcript_demo(app_demo, shutdown_demo));
        tokio::spawn(run_mic_level_monitor(app_mic, capture_mic, shutdown_mic));
        return Ok(SttSessionHandle {
            jwt_tx,
            shutdown: shutdown_tx,
        });
    }

    let app2 = app.clone();
    let capture_stt = capture.clone();
    tokio::spawn(async move {
        if let Err(e) = run_stt_with_audio(
            app2,
            endpoint,
            jwt_rx,
            transcription_config,
            capture_stt,
            shutdown_rx,
        )
        .await
        {
            tracing::error!(error = %e, "STT session ended");
        }
    });

    Ok(SttSessionHandle {
        jwt_tx,
        shutdown: shutdown_tx,
    })
}

async fn run_mic_level_monitor(
    app: AppHandle,
    capture: Arc<Mutex<aniki_audio_core::DualCapture>>,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) {
    while !*shutdown.borrow() {
        let level = {
            let cap = capture.lock().await;
            cap.mic_level()
        };
        let system_active = {
            let cap = capture.lock().await;
            cap.system_capturing()
        };
        let _ = app.emit(
            "mic-level",
            json!({
                "level": level,
                "active": level > 0.001,
                "systemActive": system_active,
            })
            .to_string(),
        );
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    let mut cap = capture.lock().await;
    cap.stop();
}

async fn run_stt_with_audio(
    app: AppHandle,
    endpoint: String,
    mut jwt_rx: mpsc::Receiver<String>,
    transcription_config: serde_json::Value,
    capture: Arc<Mutex<aniki_audio_core::DualCapture>>,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) -> Result<(), String> {
    let mut jwt = jwt_rx.recv().await.ok_or("missing jwt")?;

    loop {
        if *shutdown.borrow() {
            break;
        }

        let url = format!("{endpoint}?jwt={jwt}");
        let connect = connect_async(&url).await;
        let Ok((ws, _)) = connect else {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            if let Ok(new_jwt) = jwt_rx.try_recv() {
                jwt = new_jwt;
            }
            continue;
        };

        let (mut write, mut read) = ws.split();
        let start = StartRecognition {
            message: "StartRecognition",
            audio_format: AudioFormat::default(),
            transcription_config: Some(transcription_config.clone()),
        };
        write
            .send(Message::Text(serde_json::to_string(&start).unwrap().into()))
            .await
            .map_err(|e| e.to_string())?;

        loop {
            if *shutdown.borrow() {
                break;
            }

            while let Ok(new_jwt) = jwt_rx.try_recv() {
                jwt = new_jwt;
                break;
            }

            tokio::select! {
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => handle_stt_message(&app, &text),
                        Some(Ok(Message::Close(_))) | None => break,
                        Some(Err(e)) => return Err(e.to_string()),
                        _ => {}
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                    let (pcm, level, system_active) = {
                        let mut cap = capture.lock().await;
                        let level = cap.mic_level();
                        let system_active = cap.system_capturing();
                        let pcm = cap.drain_interleaved_16k();
                        (pcm, level, system_active)
                    };
                    let _ = app.emit(
                        "mic-level",
                        json!({
                            "level": level,
                            "active": level > 0.001,
                            "systemActive": system_active,
                        })
                        .to_string(),
                    );
                    if !pcm.is_empty() {
                        let bytes: Vec<u8> = pcm.iter().flat_map(|s| s.to_le_bytes()).collect();
                        if write.send(Message::Binary(bytes.into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }

        let end = EndOfStream {
            message: "EndOfStream",
        };
        let _ = write
            .send(Message::Text(serde_json::to_string(&end).unwrap().into()))
            .await;

        if *shutdown.borrow() {
            break;
        }
    }

    {
        let mut cap = capture.lock().await;
        cap.stop();
    }

    Ok(())
}

fn handle_stt_message(app: &AppHandle, text: &str) {
    let Ok(msg) = serde_json::from_str::<SttServerMessage>(text) else {
        return;
    };

    let (results, is_final) = match msg {
        SttServerMessage::AddPartialTranscript { results, .. } => (results, false),
        SttServerMessage::AddTranscript { results, .. } => (results, true),
        SttServerMessage::Error { reason } => {
            tracing::error!(%reason, "Speechmatics error");
            return;
        }
        _ => return,
    };

    for (i, result) in results.iter().enumerate() {
        if let Some(alt) = result.alternatives.first() {
            let speaker = map_speaker(result.channel.as_deref(), alt.speaker.as_deref());
            let payload = json!({
                "id": format!("{}-{}", if is_final { "f" } else { "p" }, i),
                "speaker": speaker,
                "text": alt.content,
                "isFinal": is_final,
            });
            let _ = app.emit("transcript-update", payload.to_string());
        }
    }
}

async fn run_dev_transcript_demo(app: AppHandle, mut shutdown: tokio::sync::watch::Receiver<bool>) {
    let demos = [
        (
            "interviewer",
            "Can you walk me through your experience with distributed systems?",
            true,
        ),
        (
            "candidate",
            "Sure, I worked on event-driven microservices at my last role.",
            true,
        ),
        (
            "interviewer",
            "How did you handle eventual consistency?",
            true,
        ),
    ];
    for (speaker, text, is_final) in demos {
        if *shutdown.borrow() {
            break;
        }
        let payload = json!({
            "id": format!("dev-{speaker}"),
            "speaker": speaker,
            "text": text,
            "isFinal": is_final,
        });
        let _ = app.emit("transcript-update", payload.to_string());
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
