import { useState } from "react";
import { Navigate } from "react-router-dom";
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle } from "@aniki/ui";
import { api } from "../lib/api";
import { useAuth } from "../lib/auth";

export function LoginPage() {
  const { user } = useAuth();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (user) {
    return <Navigate to="/app" replace />;
  }

  const startGoogle = async () => {
    setLoading(true);
    setError(null);
    try {
      const { authorization_url } = await api.oauthGoogleStart();
      window.location.assign(authorization_url);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Google sign-in is unavailable");
      setLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center p-4">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>Sign in to Aniki</CardTitle>
          <CardDescription>
            Real-time interview assistant with resume-aware answers
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {error && <p className="text-sm text-destructive">{error}</p>}
          <Button className="w-full" onClick={() => void startGoogle()} disabled={loading}>
            {loading ? "Redirecting…" : "Continue with Google"}
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
