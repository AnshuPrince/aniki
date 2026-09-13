import { open } from "@tauri-apps/plugin-shell";
import { useCallback, useEffect, useState } from "react";
import type { Session, SessionDetailResponse } from "@aniki/shared";
import { Button } from "@aniki/ui";
import { api } from "../../lib/api";
import { WEB_URL } from "../constants";
import { useOverlay } from "../context/OverlayContext";

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString(undefined, {
    day: "numeric",
    month: "short",
    year: "numeric",
  });
}

function sessionDuration(session: Session) {
  if (!session.ended_at) return "—";
  const ms = new Date(session.ended_at).getTime() - new Date(session.started_at).getTime();
  const mins = Math.floor(ms / 60000);
  const secs = Math.floor((ms % 60000) / 1000);
  return `${mins}m ${secs}s`;
}

function sessionTitle(session: Session) {
  if (!session.extra_context) return "Session";
  const company = session.extra_context.match(/Company:\s*(.+)/)?.[1];
  return company?.trim() || session.extra_context.slice(0, 40);
}

export function SessionsTab() {
  const { joinSession, starting } = useOverlay();
  const [sessions, setSessions] = useState<Session[]>([]);
  const [search, setSearch] = useState("");
  const [joinId, setJoinId] = useState("");
  const [selected, setSelected] = useState<SessionDetailResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    const data = await api.listSessions();
    setSessions(data.sessions);
  }, []);

  useEffect(() => {
    load()
      .catch(console.error)
      .finally(() => setLoading(false));
  }, [load]);

  const filtered = sessions.filter((s) => {
    const q = search.toLowerCase();
    const hay = `${s.extra_context ?? ""} ${s.id}`.toLowerCase();
    return hay.includes(q);
  });

  if (selected) {
    return (
      <div className="flex min-h-0 flex-1 flex-col px-3 pb-4">
        <button
          type="button"
          onClick={() => setSelected(null)}
          className="mb-2 text-left text-xs text-primary"
        >
          ← Back
        </button>
        <h3 className="mb-2 text-sm font-medium">{sessionTitle(selected.session)}</h3>
        <p className="mb-2 text-xs capitalize text-muted-foreground">{selected.session.status}</p>
        {selected.notes?.summary && (
          <div className="mb-3 rounded-xl border border-white/10 bg-white/5 p-3 text-sm">
            <p className="mb-1 text-xs font-medium text-primary">Summary</p>
            <p className="whitespace-pre-wrap text-xs">{selected.notes.summary}</p>
          </div>
        )}
        {selected.transcript && (
          <div className="min-h-0 flex-1 overflow-y-auto rounded-xl border border-white/10 bg-white/5 p-3 text-xs whitespace-pre-wrap">
            {selected.transcript}
          </div>
        )}
      </div>
    );
  }

  return (
    <div className="flex min-h-0 flex-1 flex-col px-3 pb-4">
      <div className="relative mb-3">
        <input
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search by title or description"
          className="w-full rounded-xl border border-white/10 bg-white/5 py-2 pl-3 pr-8 text-sm"
        />
        <span className="absolute right-3 top-2.5 text-muted-foreground">⌕</span>
      </div>

      <div className="mb-3 flex gap-2">
        <input
          value={joinId}
          onChange={(e) => setJoinId(e.target.value)}
          placeholder="Paste active session ID"
          className="min-w-0 flex-1 rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-xs"
        />
        <Button
          size="sm"
          disabled={starting || !joinId.trim()}
          onClick={() => {
            setError(null);
            void joinSession(joinId)
              .then(() => setJoinId(""))
              .catch((e) =>
                setError(e instanceof Error ? e.message : "Failed to join session"),
              );
          }}
        >
          Join
        </Button>
      </div>

      <div className="min-h-0 flex-1 space-y-2 overflow-y-auto">
        {loading && <p className="text-xs text-muted-foreground">Loading…</p>}
        {!loading && filtered.length === 0 && (
          <p className="text-xs text-muted-foreground">No sessions yet.</p>
        )}
        {filtered.map((s) => (
          <div key={s.id} className="rounded-xl border border-white/10 bg-white/5 p-3">
            <div className="mb-1 flex items-center justify-between">
              <span className="text-[10px] uppercase text-muted-foreground">
                {formatDate(s.started_at)}
              </span>
              <span className="text-[10px] capitalize text-muted-foreground">{s.status}</span>
            </div>
            <p className="text-sm font-medium">{sessionTitle(s)}</p>
            <p className="mb-2 truncate text-xs text-muted-foreground">
              {s.extra_context?.slice(0, 80) ?? s.id}
            </p>
            <div className="mb-2 flex gap-1">
              <span className="rounded-full bg-white/10 px-2 py-0.5 text-[10px]">
                {s.enable_screen_ocr ? "Interview" : "Call"}
              </span>
              <span className="rounded-full bg-white/10 px-2 py-0.5 text-[10px]">Transcript</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-[10px] text-muted-foreground">{sessionDuration(s)}</span>
              <div className="flex gap-1">
                {s.status === "active" && (
                  <Button
                    size="sm"
                    className="h-7 text-xs"
                    disabled={starting}
                    onClick={() => {
                      setError(null);
                      void joinSession(s.id).catch((e) =>
                        setError(e instanceof Error ? e.message : "Failed to join session"),
                      );
                    }}
                  >
                    {starting ? "Joining…" : "Join"}
                  </Button>
                )}
                <Button
                  size="sm"
                  variant="ghost"
                  className="h-7 text-xs"
                  onClick={() => api.getSession(s.id).then(setSelected)}
                >
                  View
                </Button>
              </div>
            </div>
          </div>
        ))}
      </div>

      {error && <p className="mt-2 text-xs text-destructive">{error}</p>}

      <Button
        variant="outline"
        className="mt-3 w-full rounded-xl"
        onClick={() => void open(`${WEB_URL}/app/sessions`)}
      >
        View in Dashboard ↗
      </Button>
    </div>
  );
}
