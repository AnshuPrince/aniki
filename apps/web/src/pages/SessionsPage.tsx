import { useCallback, useEffect, useState } from "react";
import type {
  CreateSessionResponse,
  LlmModel,
  Resume,
  Session,
  SessionDetailResponse,
} from "@aniki/shared";
import { ApiError } from "@aniki/shared";
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@aniki/ui";
import { api, TOKEN_KEY } from "../lib/api";
import { useAuth } from "../lib/auth";

const MODEL_OPTIONS: { value: LlmModel; label: string }[] = [
  { value: "gpt41", label: "GPT-4.1" },
  { value: "claude_sonnet", label: "Claude Sonnet" },
  { value: "gpt41_mini", label: "GPT-4.1 Mini" },
];

function formatModel(model: LlmModel) {
  return MODEL_OPTIONS.find((m) => m.value === model)?.label ?? model;
}

export function SessionsPage() {
  const { user, refreshUser } = useAuth();
  const [sessions, setSessions] = useState<Session[]>([]);
  const [resumes, setResumes] = useState<Resume[]>([]);
  const [selected, setSelected] = useState<SessionDetailResponse | null>(null);
  const [activeSession, setActiveSession] = useState<CreateSessionResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [starting, setStarting] = useState(false);
  const [ending, setEnding] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [copied, setCopied] = useState<string | null>(null);

  const [resumeId, setResumeId] = useState("");
  const [model, setModel] = useState<LlmModel>("gpt41");
  const [enableOcr, setEnableOcr] = useState(false);

  const loadSessions = useCallback(async () => {
    const data = await api.listSessions();
    setSessions(data.sessions);
  }, []);

  const activeFromList = sessions.find((s) => s.status === "active");
  const activeSessionId = activeSession?.session.id ?? activeFromList?.id;

  useEffect(() => {
    Promise.all([loadSessions(), api.listResumes().then((r) => setResumes(r.resumes))])
      .catch((e) => setError(e instanceof Error ? e.message : "Failed to load sessions"))
      .finally(() => setLoading(false));
  }, [loadSessions]);

  const openSession = useCallback(async (id: string) => {
    const detail = await api.getSession(id);
    setSelected(detail);
    return detail;
  }, []);

  useEffect(() => {
    if (!selected?.session.id) return;
    if (selected.notes?.summary) return;
    if (selected.session.status !== "ended") return;

    const interval = setInterval(() => {
      openSession(selected.session.id).catch(console.error);
    }, 4000);

    return () => clearInterval(interval);
  }, [selected, openSession]);

  const copyText = async (label: string, value: string) => {
    await navigator.clipboard.writeText(value);
    setCopied(label);
    setTimeout(() => setCopied(null), 2000);
  };

  const startSession = async () => {
    setError(null);
    setStarting(true);
    try {
      const response = await api.createSession({
        resume_id: resumeId || undefined,
        model,
        enable_screen_ocr: enableOcr,
      });
      setActiveSession(response);
      await loadSessions();
      await refreshUser();
      await openSession(response.session.id);
    } catch (e) {
      if (e instanceof ApiError && e.status === 402) {
        setError("Not enough credits. Each session costs 0.5 credits.");
      } else {
        setError(e instanceof Error ? e.message : "Failed to start session");
      }
    } finally {
      setStarting(false);
    }
  };

  const endActiveSession = async () => {
    const id = activeSessionId;
    if (!id) return;
    setEnding(true);
    setError(null);
    try {
      await api.finalizeSession(id);
      setActiveSession(null);
      await loadSessions();
      await refreshUser();
      await openSession(id);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to end session");
    } finally {
      setEnding(false);
    }
  };

  const accessToken = localStorage.getItem(TOKEN_KEY) ?? "";

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-3xl font-bold">Sessions</h1>
          <p className="text-muted-foreground">
            Start interview sessions here, then join from the desktop overlay
          </p>
        </div>
      </div>

      {error && (
        <div className="rounded-md border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive">
          {error}
        </div>
      )}

      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Start a session</CardTitle>
          <CardDescription>
            Costs 0.5 credits · You have {user?.credits?.toFixed(1) ?? "—"} credits
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <label className="block space-y-2 text-sm">
              <span className="font-medium">Resume (optional)</span>
              <select
                value={resumeId}
                onChange={(e) => setResumeId(e.target.value)}
                className="w-full rounded-md border border-input bg-background px-3 py-2"
                disabled={!!activeSessionId}
              >
                <option value="">No resume</option>
                {resumes.map((r) => (
                  <option key={r.id} value={r.id}>
                    {r.filename} ({r.status})
                  </option>
                ))}
              </select>
            </label>

            <label className="block space-y-2 text-sm">
              <span className="font-medium">Answer model</span>
              <select
                value={model}
                onChange={(e) => setModel(e.target.value as LlmModel)}
                className="w-full rounded-md border border-input bg-background px-3 py-2"
                disabled={!!activeSessionId}
              >
                {MODEL_OPTIONS.map((opt) => (
                  <option key={opt.value} value={opt.value}>
                    {opt.label}
                  </option>
                ))}
              </select>
            </label>
          </div>

          <label className="flex items-center gap-2 text-sm">
            <input
              type="checkbox"
              checked={enableOcr}
              onChange={(e) => setEnableOcr(e.target.checked)}
              disabled={!!activeSessionId}
            />
            Enable screen OCR (coding interviews)
          </label>

          {!activeSessionId ? (
            <Button onClick={startSession} disabled={starting || (user?.credits ?? 0) < 0.5}>
              {starting ? "Starting…" : "Start session"}
            </Button>
          ) : (
            <div className="space-y-4 rounded-lg border border-primary/30 bg-primary/5 p-4">
              <div className="flex flex-wrap items-center justify-between gap-2">
                <p className="text-sm font-medium text-primary">Session active</p>
                <Button variant="destructive" size="sm" onClick={endActiveSession} disabled={ending}>
                  {ending ? "Ending…" : "End session"}
                </Button>
              </div>
              <p className="text-sm text-muted-foreground">
                Open the desktop app and paste your access token to connect the overlay to this
                session.
              </p>
              <div className="grid gap-2 sm:grid-cols-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => copyText("token", accessToken)}
                >
                  {copied === "token" ? "Copied!" : "Copy access token"}
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => copyText("id", activeSessionId)}
                >
                  {copied === "id" ? "Copied!" : "Copy session ID"}
                </Button>
              </div>
              <p className="text-xs text-muted-foreground">
                Desktop: run <code className="rounded bg-secondary px-1">pnpm dev:desktop</code> from
                the repo root, paste the token, and click Start session in the overlay.
              </p>
            </div>
          )}
        </CardContent>
      </Card>

      <div className="grid min-h-[420px] gap-4 lg:grid-cols-2">
        <Card className="flex flex-col">
          <CardHeader>
            <CardTitle className="text-lg">History</CardTitle>
            <CardDescription>Recent interview sessions</CardDescription>
          </CardHeader>
          <CardContent className="flex-1">
            {loading ? (
              <p className="text-sm text-muted-foreground">Loading…</p>
            ) : sessions.length === 0 ? (
              <p className="text-sm text-muted-foreground">
                No sessions yet. Use the form above to start your first session.
              </p>
            ) : (
              <ul className="divide-y divide-border">
                {sessions.map((s) => (
                  <li key={s.id}>
                    <button
                      type="button"
                      onClick={() => openSession(s.id)}
                      className="flex w-full justify-between gap-4 py-3 text-left text-sm hover:text-primary"
                    >
                      <span>
                        <span className="block font-medium">
                          {new Date(s.started_at).toLocaleString()}
                        </span>
                        <span className="text-xs text-muted-foreground">
                          {formatModel(s.model)}
                        </span>
                      </span>
                      <span className="shrink-0 capitalize text-muted-foreground">{s.status}</span>
                    </button>
                  </li>
                ))}
              </ul>
            )}
          </CardContent>
        </Card>

        <Card className="flex flex-col">
          <CardHeader>
            <CardTitle className="text-lg">Session detail</CardTitle>
            <CardDescription>Notes and transcript for the selected session</CardDescription>
          </CardHeader>
          <CardContent className="flex-1">
            {!selected ? (
              <p className="text-sm text-muted-foreground">Select a session to view notes.</p>
            ) : (
              <div className="space-y-4 text-sm">
                <div>
                  <p className="font-medium">Summary</p>
                  <p className="text-muted-foreground">
                    {selected.notes?.summary ??
                      (selected.session.status === "ended"
                        ? "Generating notes…"
                        : "Notes available after session ends.")}
                  </p>
                </div>
                {selected.notes?.key_points?.length ? (
                  <div>
                    <p className="font-medium">Key points</p>
                    <ul className="list-disc space-y-1 pl-5 text-muted-foreground">
                      {selected.notes.key_points.map((p) => (
                        <li key={p}>{p}</li>
                      ))}
                    </ul>
                  </div>
                ) : null}
                {selected.transcript ? (
                  <div>
                    <p className="font-medium">Transcript excerpt</p>
                    <pre className="mt-1 max-h-48 overflow-auto rounded bg-secondary p-2 text-xs whitespace-pre-wrap">
                      {selected.transcript.slice(0, 2000)}
                    </pre>
                  </div>
                ) : null}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
