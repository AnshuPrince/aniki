import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@aniki/ui";
import { useAuth } from "../lib/auth";

export function DashboardPage() {
  const { user } = useAuth();

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold">Welcome back</h1>
        <p className="text-muted-foreground">
          Manage resumes, configure sessions, and download the desktop overlay.
        </p>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Credits</CardTitle>
            <CardDescription>Available session credits</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-3xl font-bold text-primary">{user?.credits?.toFixed(1)}</p>
            <p className="text-xs text-muted-foreground mt-1">0.5 credit = 30 min session</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Desktop App</CardTitle>
            <CardDescription>Stealth overlay for live interviews</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Open the Aniki desktop app when you are ready to join a session.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Quick Start</CardTitle>
            <CardDescription>Get ready for your next interview</CardDescription>
          </CardHeader>
          <CardContent>
            <ol className="list-decimal list-inside space-y-1 text-sm text-muted-foreground">
              <li>Upload your resume</li>
              <li>
                <a href="/app/sessions" className="text-primary hover:underline">
                  Start a session
                </a>{" "}
                on the Sessions page
              </li>
              <li>Open the desktop overlay for live answers</li>
            </ol>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
