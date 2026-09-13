import { useEffect, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@aniki/ui";
import { useAuth } from "../lib/auth";

export function VerifyPage() {
  const [searchParams] = useSearchParams();
  const { verify } = useAuth();
  const navigate = useNavigate();
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const token = searchParams.get("token");
    if (!token) {
      setError("Missing verification token");
      return;
    }

    verify(token)
      .then(() => navigate("/app"))
      .catch((err) => setError(err instanceof Error ? err.message : "Verification failed"));
  }, [searchParams, verify, navigate]);

  return (
    <div className="flex min-h-screen items-center justify-center p-4">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>Verifying...</CardTitle>
          <CardDescription>Signing you in</CardDescription>
        </CardHeader>
        <CardContent>
          {error ? (
            <p className="text-sm text-destructive">{error}</p>
          ) : (
            <p className="text-sm text-muted-foreground">Please wait...</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
