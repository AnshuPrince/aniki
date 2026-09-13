import { useEffect, useState } from "react";
import { Link, useNavigate, useSearchParams } from "react-router-dom";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@aniki/ui";
import { useAuth } from "../lib/auth";

export function OAuthCallbackPage() {
  const [searchParams] = useSearchParams();
  const { completeGoogle } = useAuth();
  const navigate = useNavigate();
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const code = searchParams.get("code");
    const state = searchParams.get("state");
    const oauthError = searchParams.get("error");
    if (oauthError) {
      setError("Google sign-in was cancelled or denied.");
      return;
    }
    if (!code || !state) {
      setError("Missing sign-in details. Please try again.");
      return;
    }

    completeGoogle(code, state)
      .then(() => navigate("/app", { replace: true }))
      .catch((err) => setError(err instanceof Error ? err.message : "Sign-in failed"));
  }, [searchParams, completeGoogle, navigate]);

  return (
    <div className="flex min-h-screen items-center justify-center p-4">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>{error ? "Sign-in failed" : "Signing you in"}</CardTitle>
          <CardDescription>
            {error ? "We could not complete Google sign-in." : "Please wait…"}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {error ? (
            <div className="space-y-3">
              <p className="text-sm text-destructive">{error}</p>
              <Link to="/login" className="text-sm text-primary hover:underline">
                Back to sign in
              </Link>
            </div>
          ) : (
            <p className="text-sm text-muted-foreground">Contacting Aniki…</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
