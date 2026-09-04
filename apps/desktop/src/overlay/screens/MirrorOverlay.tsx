import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { GlassPanel } from "../chrome/GlassPanel";
import { ResizeHandle } from "../chrome/ResizeHandle";
import type { TranscriptLine } from "../types";

export function MirrorOverlay() {
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
    <div className="relative flex h-screen w-screen flex-col p-2">
      <GlassPanel className="relative flex min-h-0 flex-1 flex-col overflow-hidden">
        <div className="border-b border-white/10 px-3 py-2 text-sm font-bold text-primary">
          Aniki Mirror
        </div>
        <div className="grid min-h-0 flex-1 grid-rows-2 gap-2 p-3">
          <div className="overflow-y-auto rounded-xl border border-white/10 bg-white/5 p-3">
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
          <div className="overflow-y-auto rounded-xl border border-primary/30 bg-white/5 p-3">
            <p className="mb-2 text-xs font-medium text-primary">Suggested answer</p>
            <p className="text-sm whitespace-pre-wrap">{answer || "Waiting for answers…"}</p>
          </div>
        </div>
        <ResizeHandle />
      </GlassPanel>
    </div>
  );
}
