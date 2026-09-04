import { open } from "@tauri-apps/plugin-shell";
import { useState } from "react";
import { Button, Input } from "@aniki/ui";
import { api } from "../../lib/api";
import { WEB_URL } from "../constants";
import { GlassPanel } from "../chrome/GlassPanel";
import { OverlayHeader } from "../chrome/OverlayHeader";
import { ResizeHandle } from "../chrome/ResizeHandle";
import { useOverlay } from "../context/OverlayContext";

type LoginMode = "token" | "email";

export function LoginScreen() {
  const { collapse, login, loginWithAccessToken } = useOverlay();
  const [mode, setMode] = useState<LoginMode>("token");
  const [accessToken, setAccessTokenInput] = useState("");
  const [email, setEmail] = useState("");
  const [verifyToken, setVerifyToken] = useState("");
  const [emailSent, setEmailSent] = useState(false);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
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

  const sendMagicLink = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    setMessage(null);
    try {
      await api.requestMagicLink(email.trim());
      setEmailSent(true);
      setMessage("Link generated. Copy the token= value from the API logs without opening it.");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to send magic link");
    } finally {
      setLoading(false);
    }
  };

  const submitVerifyToken = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      await login(verifyToken.trim());
    } catch {
      setError("Token invalid, expired, or already used by the browser.");
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

          <div className="mb-4 flex rounded-xl bg-white/5 p-1">
            <button
              type="button"
              onClick={() => setMode("token")}
              className={`flex-1 rounded-lg py-1.5 text-xs font-medium ${
                mode === "token" ? "bg-white/10" : "text-muted-foreground"
              }`}
            >
              Access token
            </button>
            <button
              type="button"
              onClick={() => setMode("email")}
              className={`flex-1 rounded-lg py-1.5 text-xs font-medium ${
                mode === "email" ? "bg-white/10" : "text-muted-foreground"
              }`}
            >
              Magic link
            </button>
          </div>

          {mode === "token" ? (
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
              {error && <p className="text-xs text-destructive">{error}</p>}
              <Button type="submit" className="w-full" disabled={loading}>
                {loading ? "Signing in…" : "Login"}
              </Button>
            </form>
          ) : !emailSent ? (
            <form onSubmit={sendMagicLink} className="w-full space-y-3">
              <Input
                type="email"
                placeholder="you@example.com"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                className="bg-white/5"
              />
              <p className="text-xs text-muted-foreground">
                In dev the link is printed to the API logs instead of emailed.
              </p>
              {error && <p className="text-xs text-destructive">{error}</p>}
              <Button type="submit" className="w-full" disabled={loading}>
                {loading ? "Sending…" : "Send magic link"}
              </Button>
            </form>
          ) : (
            <form onSubmit={submitVerifyToken} className="w-full space-y-3">
              <Input
                placeholder="Paste token= value from the link"
                value={verifyToken}
                onChange={(e) => setVerifyToken(e.target.value)}
                required
                className="bg-white/5"
              />
              {message && <p className="text-xs text-muted-foreground">{message}</p>}
              {error && <p className="text-xs text-destructive">{error}</p>}
              <Button type="submit" className="w-full" disabled={loading}>
                {loading ? "Logging in…" : "Login"}
              </Button>
              <Button
                type="button"
                variant="ghost"
                className="w-full"
                onClick={() => {
                  setEmailSent(false);
                  setError(null);
                  setMessage(null);
                }}
              >
                Back
              </Button>
            </form>
          )}

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
