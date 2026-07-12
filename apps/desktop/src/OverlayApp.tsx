import { useCallback, useEffect, useRef, useState, type ReactNode } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { emit, listen } from "@tauri-apps/api/event";
import { Button } from "@aniki/ui";
import type { CreateSessionRequest, CreateSessionResponse, LlmModel } from "@aniki/shared";
import { STT_JWT_REFRESH_INTERVAL_MS } from "@aniki/shared";
import { api, setAccessToken } from "./lib/api";
import { debouncedQuestionCheck } from "./lib/questionDetect";

interface TranscriptLine {
  id: string;
  speaker: "interviewer" | "candidate";
  text: string;
  isFinal: boolean;
}

const isMirrorWindow =
  new URLSearchParams(window.location.search).get("mirror") === "1";

function OverlayTitleBar({
  left,
  actions,
}: {
  left: ReactNode;
  actions: ReactNode;
}) {
  const startDrag = (e: React.PointerEvent) => {
    if (e.button !== 0) return;
    e.preventDefault();
    void getCurrentWindow().startDragging();
  };

  return (
    <div className="mb-3 flex items-center gap-2 rounded-lg border border-border bg-card/80 px-2 py-2">
      <div
        data-tauri-drag-region
        onPointerDown={startDrag}
        className="flex min-w-0 flex-1 cursor-grab items-center gap-2 rounded-md px-1 py-0.5 active:cursor-grabbing"
        title="Drag to move"
      >
        <span className="select-none text-muted-foreground" aria-hidden>
          ⠿
        </span>
        {left}
      </div>
      <div className="flex shrink-0 gap-1">{actions}</div>
    </div>
  );
}

function MirrorOverlay() {
  const [transcript, setTranscript] = useState<TranscriptLine[]>([]);
  const [answer, setAnswer] = useState("");

  useEffect(() => {
    const unlistenTranscript = listen<string>("transcript-update", (event) => {
      try {
        const data = JSON.parse(event.payload) as TranscriptLine;
        setTranscript((prev) => {
          const existing = prev.findIndex((t) => t.id === data.id);
          if (existing >= 0) {
            const next = [...prev];
            next[existing] = data;
            return next;
          }
          return [...prev, data];
        });
      } catch {
        // ignore
      }
    });

    const unlistenAnswer = listen<string>("answer-update", (event) => {
      setAnswer(event.payload);
    });

    return () => {
      unlistenTranscript.then((fn) => fn());
      unlistenAnswer.then((fn) => fn());
    };
  }, []);

  return (
    <div className="h-screen w-screen overflow-hidden bg-background/90 backdrop-blur-md p-4">
      <OverlayTitleBar
        left={<span className="text-sm font-bold text-primary">Aniki</span>}
        actions={
          <Button size="sm" variant="ghost" onClick={() => invoke("hide_overlay")}>
            Hide
          </Button>
        }
      />
      <div className="grid h-[calc(100%-3rem)] grid-rows-2 gap-3">
        <div className="overflow-y-auto rounded-lg border border-border bg-card/80 p-3">
          <p className="mb-2 text-xs font-medium text-muted-foreground">Live transcript</p>
          {transcript.length === 0 ? (
            <p className="text-xs text-muted-foreground">Listening…</p>
          ) : (
            <ul className="space-y-1">
              {transcript.map((line) => (
                <li key={line.id} className="text-sm">
                  <span
                    className={
                      line.speaker === "interviewer" ? "text-primary" : "text-muted-foreground"
                    }
                  >
                    {line.speaker}:
                  </span>{" "}
                  {line.text}
                </li>
              ))}
            </ul>
          )}
        </div>
        <div className="overflow-y-auto rounded-lg border border-primary/30 bg-card/80 p-3">
          <p className="mb-2 text-xs font-medium text-primary">Suggested answer</p>
          <p className="text-sm whitespace-pre-wrap">{answer || "Waiting for answers…"}</p>
        </div>
      </div>
    </div>
  );
}

export function OverlayApp() {
  if (isMirrorWindow) {
    return <MirrorOverlay />;
  }

  const [token, setToken] = useState("");
  const [sessionIdInput, setSessionIdInput] = useState("");
  const [resumeId, setResumeId] = useState("");
  const [model, setModel] = useState<LlmModel>("gpt41");
  const [enableOcr, setEnableOcr] = useState(false);
  const [session, setSession] = useState<CreateSessionResponse | null>(null);
  const [transcript, setTranscript] = useState<TranscriptLine[]>([]);
  const [answer, setAnswer] = useState("");
  const [status, setStatus] = useState("idle");
  const [micLevel, setMicLevel] = useState(0);
  const [micActive, setMicActive] = useState(false);
  const [devSttMode, setDevSttMode] = useState(false);
  const [stealthWarning, setStealthWarning] = useState<string | null>(null);
  const [clickThrough, setClickThrough] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const refreshTimer = useRef<ReturnType<typeof setInterval> | null>(null);
  const lastQuestionHash = useRef<string | null>(null);
  const finalSegments = useRef<TranscriptLine[]>([]);

  useEffect(() => {
    invoke<{ capture_exclusion_degraded: boolean; degradation_reason?: string }>(
      "get_stealth_status"
    ).then((s) => {
      if (s.capture_exclusion_degraded && s.degradation_reason) {
        setStealthWarning(s.degradation_reason);
      }
    });
  }, []);

  useEffect(() => {
    invoke<boolean>("get_click_through")
      .then(setClickThrough)
      .catch(() => {});

    const unlistenClickThrough = listen<boolean>("click-through-changed", (event) => {
      setClickThrough(event.payload);
    });

    return () => {
      unlistenClickThrough.then((fn) => fn());
    };
  }, []);

  useEffect(() => {
    const unlistenMic = listen<string>("mic-level", (event) => {
      try {
        const data = JSON.parse(event.payload) as { level: number; active: boolean };
        setMicLevel(data.level);
        setMicActive(data.active);
      } catch {
        // ignore
      }
    });

    return () => {
      unlistenMic.then((fn) => fn());
    };
  }, []);

  const streamAnswerForQuestion = useCallback(
    async (question: string) => {
      if (!session) return;
      setAnswer("");
      void emit("answer-update", "");
      try {
        let ocrContext: string | undefined;
        if (session.session.enable_screen_ocr) {
          ocrContext = await invoke<string>("capture_screen_ocr");
        }
        const transcriptContext = finalSegments.current
          .slice(-6)
          .map((t) => `[${t.speaker}] ${t.text}`)
          .join("\n");

        const url = new URL(`${import.meta.env.VITE_API_URL ?? "http://localhost:8080"}/sessions/${session.session.id}/answer`);
        if (ocrContext) url.searchParams.set("screen_ocr", ocrContext);

        const token = localStorage.getItem("aniki_access_token");
        const response = await fetch(url, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
          },
          body: JSON.stringify({ question, transcript_context: transcriptContext }),
        });

        if (!response.ok || !response.body) throw new Error("Answer stream failed");

        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        let buffer = "";
        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          buffer += decoder.decode(value, { stream: true });
          const lines = buffer.split("\n");
          buffer = lines.pop() ?? "";
          for (const line of lines) {
            if (line.startsWith("data: ")) {
              const chunk = line.slice(6);
              if (chunk) {
                setAnswer((prev) => {
                  const next = prev + chunk;
                  void emit("answer-update", next);
                  return next;
                });
              }
            }
          }
        }
      } catch (e) {
        const message = e instanceof Error ? e.message : "Failed to get answer";
        setAnswer(message);
        void emit("answer-update", message);
      }
    },
    [session]
  );

  useEffect(() => {
    const unlisten = listen<string>("transcript-update", (event) => {
      try {
        const data = JSON.parse(event.payload) as TranscriptLine;
        setTranscript((prev) => {
          const existing = prev.findIndex((t) => t.id === data.id);
          if (existing >= 0) {
            const next = [...prev];
            next[existing] = data;
            return next;
          }
          return [...prev, data];
        });

        if (data.isFinal) {
          finalSegments.current = [...finalSegments.current, data].slice(-50);
        }

        debouncedQuestionCheck(data.id, data.speaker, data.text, data.isFinal, async (question) => {
          const hash = question.trim().toLowerCase();
          if (lastQuestionHash.current === hash) return;

          try {
            const { is_question } = await api.confirmQuestion(question);
            if (!is_question) return;
          } catch {
            // Fall back to client heuristic when API confirm is unavailable.
          }

          lastQuestionHash.current = hash;
          streamAnswerForQuestion(question);
        });
      } catch {
        // ignore
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [streamAnswerForQuestion]);

  const startSession = useCallback(async () => {
    if (!token.trim()) {
      setError("Paste your access token from the web dashboard");
      return;
    }

    setError(null);
    setStatus("starting");

    try {
      setAccessToken(token.trim());
      const req: CreateSessionRequest = {
        model,
        enable_screen_ocr: enableOcr,
        resume_id: resumeId.trim() || undefined,
      };

      let response: CreateSessionResponse;
      const existingId = sessionIdInput.trim();
      if (existingId) {
        const detail = await api.getSession(existingId);
        if (detail.session.status !== "active") {
          throw new Error("That session is not active. Start a new one on the web Sessions page.");
        }
        const stt = await api.refreshSttJwt(existingId);
        response = {
          session: detail.session,
          stt_jwt: stt.jwt,
          stt_endpoint: stt.endpoint,
          stt_expires_at: stt.expires_at,
        };
      } else {
        response = await api.createSession(req);
      }

      setSession(response);

      const audioStatus = await invoke<{
        mic_active: boolean;
        dev_stt_mode: boolean;
        live_stt: boolean;
      }>("start_audio_session", {
        sttJwt: response.stt_jwt,
        sttEndpoint: response.stt_endpoint,
      });

      setMicActive(audioStatus.mic_active);
      setDevSttMode(audioStatus.dev_stt_mode);

      refreshTimer.current = setInterval(async () => {
        try {
          const refreshed = await api.refreshSttJwt(response.session.id);
          await invoke("refresh_stt_jwt", { jwt: refreshed.jwt });
        } catch (e) {
          console.error("STT JWT refresh failed", e);
        }
      }, STT_JWT_REFRESH_INTERVAL_MS);

      setStatus("active");
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to start session");
      setStatus("idle");
    }
  }, [token, sessionIdInput, model, enableOcr, resumeId]);

  const endSession = useCallback(async () => {
    if (refreshTimer.current) {
      clearInterval(refreshTimer.current);
      refreshTimer.current = null;
    }

    if (session) {
      const segments = finalSegments.current.map((s) => ({
        id: s.id,
        speaker: s.speaker,
        text: s.text,
        is_final: s.isFinal,
      }));
      await api.appendTranscript(session.session.id, segments).catch(console.error);
      await api.finalizeSession(session.session.id).catch(console.error);
    }

    await invoke("stop_audio_session").catch(console.error);
    setSession(null);
    setStatus("idle");
    setMicLevel(0);
    setMicActive(false);
    setDevSttMode(false);
    setTranscript([]);
    setAnswer("");
    finalSegments.current = [];
  }, [session]);

  return (
    <div className="h-screen w-screen overflow-hidden bg-background/90 backdrop-blur-md p-4">
      <OverlayTitleBar
        left={
          <div className="flex min-w-0 items-center gap-2">
            <span className="text-sm font-bold text-primary">Aniki</span>
            <span className="rounded-full bg-secondary px-2 py-0.5 text-xs capitalize">{status}</span>
            {session && (
              <span
                className={`flex items-center gap-1 rounded-full px-2 py-0.5 text-xs ${
                  micActive ? "bg-primary/20 text-primary" : "bg-secondary text-muted-foreground"
                }`}
                title={micActive ? "Microphone receiving audio" : "Waiting for mic input"}
              >
                <span
                  className={`inline-block h-2 w-2 rounded-full ${
                    micActive ? "bg-primary animate-pulse" : "bg-muted-foreground"
                  }`}
                />
                Mic
              </span>
            )}
          </div>
        }
        actions={
          <>
            <Button
              size="sm"
              variant={clickThrough ? "secondary" : "ghost"}
              onClick={async () => {
                const enabled = await invoke<boolean>("toggle_click_through");
                setClickThrough(enabled);
              }}
            >
              {clickThrough ? "Click-through on" : "Click-through"}
            </Button>
        <Button size="sm" variant="ghost" onClick={() => invoke("hide_overlay")}>
          Hide
        </Button>
            {session ? (
              <Button size="sm" variant="destructive" onClick={endSession}>
                End
              </Button>
            ) : null}
          </>
        }
      />

      {clickThrough && (
        <p className="mb-2 text-xs text-amber-400">
          Click-through is on — mouse passes through the overlay. Press{" "}
          <kbd className="rounded border border-border px-1">⌘⇧C</kbd> to interact again, or{" "}
          <kbd className="rounded border border-border px-1">⌘⇧H</kbd> to hide and restore controls.
        </p>
      )}

      {devSttMode && session && (
        <p className="mb-2 text-xs text-amber-400">
          Dev STT mode: demo transcripts only. Set SPEECHMATICS_API_KEY on the API for live
          transcription. Microphone is still active for level monitoring.
        </p>
      )}

      {session && micLevel > 0.01 && (
        <div className="mb-2 h-1 overflow-hidden rounded-full bg-secondary">
          <div
            className="h-full bg-primary transition-all duration-100"
            style={{ width: `${Math.min(100, micLevel * 100)}%` }}
          />
        </div>
      )}

      {stealthWarning && (
        <p className="mb-2 text-xs text-amber-400">{stealthWarning}</p>
      )}

      {!session ? (
        <div className="space-y-3 rounded-lg border border-border bg-card/80 p-4">
          <input
            type="password"
            value={token}
            onChange={(e) => setToken(e.target.value)}
            placeholder="Access token (from web Sessions page)..."
            className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          />
          <input
            value={sessionIdInput}
            onChange={(e) => setSessionIdInput(e.target.value)}
            placeholder="Session ID from web (recommended)"
            className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          />
          <p className="text-xs text-muted-foreground">
            Start a session on the web, then paste the session ID here. Leave blank to create a new
            session from the overlay.
          </p>
          <input
            value={resumeId}
            onChange={(e) => setResumeId(e.target.value)}
            placeholder="Resume ID (only if creating new session)"
            className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          />
          <select
            value={model}
            onChange={(e) => setModel(e.target.value as LlmModel)}
            className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          >
            <option value="gpt41">GPT-4.1</option>
            <option value="claude_sonnet">Claude Sonnet</option>
            <option value="gpt41_mini">GPT-4.1 Mini</option>
          </select>
          <label className="flex items-center gap-2 text-sm">
            <input type="checkbox" checked={enableOcr} onChange={(e) => setEnableOcr(e.target.checked)} />
            Screen OCR (coding mode)
          </label>
          {error && <p className="text-xs text-destructive">{error}</p>}
          <Button className="w-full" onClick={startSession}>
            Start session
          </Button>
        </div>
      ) : (
        <div className="grid h-[calc(100%-3rem)] grid-rows-2 gap-3">
          <div className="overflow-y-auto rounded-lg border border-border bg-card/80 p-3">
            <p className="mb-2 text-xs font-medium text-muted-foreground">Live transcript</p>
            {transcript.length === 0 ? (
              <p className="text-xs text-muted-foreground">Listening...</p>
            ) : (
              <ul className="space-y-1">
                {transcript.map((line) => (
                  <li key={line.id} className="text-sm">
                    <span className={line.speaker === "interviewer" ? "text-primary" : "text-muted-foreground"}>
                      {line.speaker}:
                    </span>{" "}
                    {line.text}
                    {!line.isFinal && <span className="text-muted-foreground">...</span>}
                  </li>
                ))}
              </ul>
            )}
          </div>
          <div className="overflow-y-auto rounded-lg border border-primary/30 bg-card/80 p-3">
            <p className="mb-2 text-xs font-medium text-primary">Suggested answer</p>
            <p className="text-sm whitespace-pre-wrap">
              {answer || "Answers appear when interviewer questions are detected."}
            </p>
          </div>
        </div>
      )}
    </div>
  );
}
