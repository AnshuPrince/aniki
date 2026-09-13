import { useEffect, useState } from "react";
import type { LlmModel, Resume } from "@aniki/shared";
import { Button } from "@aniki/ui";
import { api } from "../../lib/api";
import { CURRENT_MODEL } from "../constants";
import { FieldLabel } from "../chrome/OverlayHeader";
import { useOverlay } from "../context/OverlayContext";

type SessionType = "interview" | "regular";

export function CreateTab() {
  const { startSession, starting, autoAnswer, setAutoAnswer } = useOverlay();
  const [sessionType, setSessionType] = useState<SessionType>("interview");
  const [company, setCompany] = useState("");
  const [description, setDescription] = useState("");
  const [documents, setDocuments] = useState("");
  const [instructions, setInstructions] = useState("");
  const [resumeId, setResumeId] = useState("");
  const [resumes, setResumes] = useState<Resume[]>([]);
  const model: LlmModel = CURRENT_MODEL.value;
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    api.listResumes().then((r) => setResumes(r.resumes)).catch(console.error);
  }, []);

  const buildExtraContext = () => {
    const parts: string[] = [];
    if (company.trim()) parts.push(`Company: ${company.trim()}`);
    if (description.trim()) parts.push(`Description: ${description.trim()}`);
    if (documents.trim()) parts.push(`Documents:\n${documents.trim()}`);
    if (instructions.trim()) parts.push(`Instructions:\n${instructions.trim()}`);
    return parts.join("\n\n");
  };

  const handleCreate = async () => {
    setError(null);
    try {
      await startSession({
        resumeId: resumeId || undefined,
        model,
        enableOcr: sessionType === "interview",
        extraContext: buildExtraContext(),
      });
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to create session");
    }
  };

  return (
    <div className="space-y-4 px-3 pb-4">
      <div className="flex rounded-xl bg-white/5 p-1">
        <button
          type="button"
          onClick={() => setSessionType("interview")}
          className={`flex flex-1 items-center justify-center gap-1.5 rounded-lg py-2 text-xs font-medium ${
            sessionType === "interview" ? "bg-white/10" : "text-muted-foreground"
          }`}
        >
          💼 Interview
        </button>
        <button
          type="button"
          onClick={() => setSessionType("regular")}
          className={`flex flex-1 items-center justify-center gap-1.5 rounded-lg py-2 text-xs font-medium ${
            sessionType === "regular" ? "bg-white/10" : "text-muted-foreground"
          }`}
        >
          📞 Regular Call
        </button>
      </div>

      <div>
        <FieldLabel hint="Company name for context">Company</FieldLabel>
        <input
          value={company}
          onChange={(e) => setCompany(e.target.value)}
          placeholder="Acme…"
          className="w-full rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-sm"
        />
        <p className="mt-1 text-[10px] text-muted-foreground/60">
          Add the company name to tailor your interview context.
        </p>
      </div>

      <div>
        <FieldLabel>Interview Description</FieldLabel>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="Software Engineer versed in Python, SQL, and AWS…"
          rows={3}
          className="w-full resize-none rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-sm"
        />
      </div>

      <div>
        <FieldLabel>Context</FieldLabel>
        <div className="flex flex-wrap gap-2">
          <select
            value={resumeId}
            onChange={(e) => setResumeId(e.target.value)}
            className="rounded-full border border-white/10 bg-white/5 px-3 py-1.5 text-xs"
          >
            <option value="">+ Resume</option>
            {resumes.filter((r) => r.status === "ready").map((r) => (
              <option key={r.id} value={r.id}>
                {r.filename}
              </option>
            ))}
          </select>
        </div>
        <textarea
          value={documents}
          onChange={(e) => setDocuments(e.target.value)}
          placeholder="+ Documents (paste notes)"
          rows={2}
          className="mt-2 w-full resize-none rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-xs"
        />
        <textarea
          value={instructions}
          onChange={(e) => setInstructions(e.target.value)}
          placeholder="+ Instructions"
          rows={2}
          className="mt-2 w-full resize-none rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-xs"
        />
      </div>

      <div>
        <FieldLabel>Output Settings</FieldLabel>
        <div className="w-full rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-sm">
          {CURRENT_MODEL.label}
        </div>
        <p className="mt-1 text-xs text-muted-foreground">Language: English</p>
      </div>

      <div>
        <FieldLabel>Behavior</FieldLabel>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            checked={autoAnswer}
            onChange={(e) => setAutoAnswer(e.target.checked)}
          />
          Auto Answer (Beta)
        </label>
        <label className="mt-1 flex items-center gap-2 text-sm text-muted-foreground">
          <input type="checkbox" checked disabled readOnly />
          Save Transcript
        </label>
      </div>

      {error && <p className="text-xs text-destructive">{error}</p>}

      <Button className="w-full rounded-xl py-5" onClick={() => void handleCreate()} disabled={starting}>
        {starting ? "Creating…" : "Create Session"}
      </Button>
    </div>
  );
}
