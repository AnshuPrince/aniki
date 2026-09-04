import { useEffect, useState } from "react";
import { Button } from "@aniki/ui";
import { GlassPanel } from "../chrome/GlassPanel";
import { IconButton } from "../chrome/IconButton";
import { ResizeHandle } from "../chrome/ResizeHandle";
import { useOverlay } from "../context/OverlayContext";

function StatusDot({ active, label }: { active: boolean; label: string }) {
  return (
    <span
      className="flex items-center gap-1 text-[10px] text-muted-foreground"
      title={label}
    >
      <span
        className={`h-2 w-2 rounded-full ${active ? "bg-red-500 animate-pulse" : "bg-muted-foreground/40"}`}
      />
      {label}
    </span>
  );
}

function WordPills({ text }: { text: string }) {
  const words = text.trim().split(/\s+/).filter(Boolean);
  if (words.length === 0) {
    return <span className="text-xs text-muted-foreground">Listening for interviewer…</span>;
  }
  return (
    <div className="flex flex-wrap gap-1">
      {words.map((word, i) => (
        <span
          key={`${word}-${i}`}
          className="rounded-full border border-white/10 bg-white/5 px-2 py-0.5 text-xs"
        >
          {word}
        </span>
      ))}
    </div>
  );
}

export function LiveOverlay() {
  const {
    collapse,
    endSession,
    streamAnswer,
    lastInterviewerQuestion,
    clearQuestion,
    answer,
    micLevel,
    micActive,
    systemActive,
    devSttMode,
    stealthWarning,
    clickThrough,
  } = useOverlay();

  const [chatOpen, setChatOpen] = useState(false);
  const [chatPrompt, setChatPrompt] = useState("");

  const triggerAnswer = () => {
    const q = lastInterviewerQuestion.trim();
    if (q) void streamAnswer(q);
  };

  const triggerScreenshot = () => {
    const q = lastInterviewerQuestion.trim() || "What is shown on screen?";
    void streamAnswer(q, true);
  };

  const submitChat = () => {
    const q = chatPrompt.trim();
    if (!q) return;
    void streamAnswer(q);
    setChatPrompt("");
    setChatOpen(false);
  };

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (!e.metaKey) return;
      if (e.key === "Enter" && e.shiftKey) {
        e.preventDefault();
        triggerScreenshot();
      } else if (e.key === "Enter") {
        e.preventDefault();
        triggerAnswer();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  });

  const copyAnswer = () => {
    if (answer) void navigator.clipboard.writeText(answer);
  };

  return (
    <div className="relative flex h-screen w-screen flex-col gap-2 p-2">
      {/* Toolbar card */}
      <GlassPanel className="shrink-0">
        <div className="flex items-center gap-2 px-3 py-2">
          <div className="flex gap-2">
            <StatusDot active={micActive} label="Mic" />
            <StatusDot active={systemActive} label="Sys" />
          </div>
          <div className="flex flex-1 justify-center gap-1">
            <Button
              size="sm"
              variant="secondary"
              className="h-7 rounded-full px-3 text-xs"
              onClick={triggerAnswer}
            >
              Answer <kbd className="ml-1 opacity-60">⌘↵</kbd>
            </Button>
            <Button
              size="sm"
              variant="secondary"
              className="h-7 rounded-full px-3 text-xs"
              onClick={triggerScreenshot}
            >
              Screenshot <kbd className="ml-1 opacity-60">⇧⌘↵</kbd>
            </Button>
            <Button
              size="sm"
              variant="secondary"
              className="h-7 rounded-full px-3 text-xs"
              onClick={() => setChatOpen((v) => !v)}
            >
              Chat
            </Button>
          </div>
          <div className="flex items-center gap-1">
            <IconButton title="Collapse" onClick={() => void collapse()}>
              <span className="text-xs">⤡</span>
            </IconButton>
            <Button
              size="sm"
              variant="destructive"
              className="h-7 rounded-full px-3 text-xs"
              onClick={() => void endSession()}
            >
              End
            </Button>
          </div>
        </div>
        {chatOpen && (
          <div className="flex gap-2 border-t border-white/10 px-3 py-2">
            <input
              value={chatPrompt}
              onChange={(e) => setChatPrompt(e.target.value)}
              placeholder="Ask anything…"
              className="flex-1 rounded-lg border border-white/10 bg-white/5 px-2 py-1 text-sm"
              onKeyDown={(e) => e.key === "Enter" && submitChat()}
            />
            <Button size="sm" onClick={submitChat}>
              Send
            </Button>
          </div>
        )}
      </GlassPanel>

      {/* Question bar */}
      <GlassPanel className="shrink-0 px-3 py-2">
        <div className="flex items-start gap-2">
          <div className="mt-1 flex h-4 w-8 shrink-0 items-end gap-0.5">
            {[0.3, 0.6, 0.4, 0.8].map((h, i) => (
              <span
                key={i}
                className="w-1 rounded-full bg-primary transition-all"
                style={{ height: `${Math.max(4, h * micLevel * 16)}px` }}
              />
            ))}
          </div>
          <div className="min-w-0 flex-1">
            <WordPills text={lastInterviewerQuestion} />
          </div>
          <Button size="sm" variant="ghost" className="h-7 shrink-0 text-xs" onClick={clearQuestion}>
            Clear
          </Button>
        </div>
      </GlassPanel>

      {/* Answer panel */}
      <GlassPanel className="relative min-h-0 flex-1 overflow-hidden">
        <div className="flex items-center justify-between border-b border-white/10 px-3 py-2">
          <span className="text-xs font-medium text-primary">★ Answer</span>
          <button
            type="button"
            onClick={copyAnswer}
            className="text-xs text-muted-foreground hover:text-foreground"
          >
            Copy
          </button>
        </div>
        <div className="overflow-y-auto px-3 py-3" style={{ maxHeight: "calc(100% - 2.5rem)" }}>
          {lastInterviewerQuestion && (
            <p className="mb-2 text-xs text-muted-foreground">
              💬 {lastInterviewerQuestion}
            </p>
          )}
          <p className="whitespace-pre-wrap text-sm">
            {answer || "Answers appear when you press Answer or a question is detected."}
          </p>
        </div>
        <ResizeHandle />
      </GlassPanel>

      {(devSttMode || !systemActive || clickThrough || stealthWarning) && (
        <div className="space-y-1 px-1">
          {devSttMode && (
            <p className="text-[10px] text-amber-400">Dev STT mode — demo transcripts only.</p>
          )}
          {!systemActive && (
            <p className="text-[10px] text-amber-400">
              Grant Screen Recording for interviewer audio.
            </p>
          )}
          {clickThrough && (
            <p className="text-[10px] text-amber-400">
              Click-through on — press ⌘⇧C from the menu to interact.
            </p>
          )}
          {stealthWarning && <p className="text-[10px] text-amber-400">{stealthWarning}</p>}
        </div>
      )}
    </div>
  );
}
