import { open } from "@tauri-apps/plugin-shell";
import { useState } from "react";
import { Button, Input } from "@aniki/ui";
import { WEB_URL } from "../constants";
import { GlassPanel } from "../chrome/GlassPanel";
import { OverlayHeader } from "../chrome/OverlayHeader";
import { ResizeHandle } from "../chrome/ResizeHandle";
import { useOverlay } from "../context/OverlayContext";

export function LoginScreen() {
  const { collapse, loginWithAccessToken } = useOverlay();
  const [accessToken, setAccessTokenInput] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const submitAccessToken = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      await loginWithAccessToken(accessToken.trim());
    } catch (err) {
      setError(err instanceof Error ? err.message : "Login failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="relative flex h-screen w-screen flex-col p-2">
      <GlassPanel className="relative flex min-h-0 flex-1 flex-col overflow-hidden">
        <OverlayHeader onCollapse={() => void collapse()} showCredits={false} />
        <div className="flex min-h-0 flex-1 flex-col justify-center overflow-y-auto px-6 py-6">
          <h1 className="mb-1 text-center text-xl font-semibold">Aniki</h1>
          <p className="mb-5 text-center text-sm text-muted-foreground">
            Login to your Aniki account to start your interview
          </p>

          <form onSubmit={submitAccessToken} className="w-full space-y-3">
            <Input
              type="password"
              placeholder="Paste access token"
              value={accessToken}
              onChange={(e) => setAccessTokenInput(e.target.value)}
              required
              className="bg-white/5"
            />
            <p className="text-xs text-muted-foreground">
              Sign in on the web dashboard, then use{" "}
              <span className="text-foreground">Copy access token</span> on the Sessions page.
            </p>
            <p className="text-xs text-muted-foreground">
              Allow Microphone and Screen Recording (system audio + OCR) in System Settings →
              Privacy &amp; Security.
            </p>
            {error && <p className="text-xs text-destructive">{error}</p>}
            <Button type="submit" className="w-full" disabled={loading}>
              {loading ? "Signing in…" : "Login"}
            </Button>
          </form>

          <button
            type="button"
            onClick={() => void open(WEB_URL)}
            className="mt-4 text-xs text-primary hover:underline"
          >
            Open web dashboard
          </button>
        </div>
        <ResizeHandle />
      </GlassPanel>
    </div>
  );
}
