import { useCallback, useEffect, useRef, useState } from "react";
import type { Resume } from "@aniki/shared";
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle, Input } from "@aniki/ui";
import { api } from "../lib/api";

export function ResumesPage() {
  const [resumes, setResumes] = useState<Resume[]>([]);
  const [loading, setLoading] = useState(true);
  const [uploading, setUploading] = useState(false);
  const fileRef = useRef<HTMLInputElement>(null);

  const loadResumes = useCallback(async () => {
    try {
      const data = await api.listResumes();
      setResumes(data.resumes);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadResumes();
  }, [loadResumes]);

  const handleUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setUploading(true);
    try {
      const arrayBuffer = await file.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);
      let binary = "";
      for (let i = 0; i < bytes.length; i++) binary += String.fromCharCode(bytes[i]!);
      const contentBase64 = btoa(binary);
      await api.uploadResume(file.name, contentBase64);
      await loadResumes();
    } finally {
      setUploading(false);
      if (fileRef.current) fileRef.current.value = "";
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold">Resumes</h1>
          <p className="text-muted-foreground">
            Upload resumes for RAG-powered interview answers
          </p>
        </div>
        <div>
          <Input
            ref={fileRef}
            type="file"
            accept=".pdf,.doc,.docx,.txt"
            className="hidden"
            onChange={handleUpload}
          />
          <Button onClick={() => fileRef.current?.click()} disabled={uploading}>
            {uploading ? "Uploading..." : "Upload resume"}
          </Button>
        </div>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Your resumes</CardTitle>
          <CardDescription>
            Resumes are chunked and embedded for context-aware answers
          </CardDescription>
        </CardHeader>
        <CardContent>
          {loading ? (
            <p className="text-sm text-muted-foreground">Loading...</p>
          ) : resumes.length === 0 ? (
            <p className="text-sm text-muted-foreground">No resumes uploaded yet.</p>
          ) : (
            <ul className="divide-y divide-border">
              {resumes.map((resume) => (
                <li key={resume.id} className="flex items-center justify-between py-3">
                  <div>
                    <p className="font-medium">{resume.filename}</p>
                    <p className="text-xs text-muted-foreground">
                      {resume.chunk_count} chunks · {resume.status}
                    </p>
                  </div>
                  <span className="rounded-full bg-secondary px-2 py-1 text-xs capitalize">
                    {resume.status}
                  </span>
                </li>
              ))}
            </ul>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
