import { Link } from "react-router-dom";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@aniki/ui";

export function PrivacyPage() {
  return (
    <div className="mx-auto max-w-2xl space-y-6 p-6">
      <Link to="/" className="text-sm text-primary hover:underline">
        Back to Aniki
      </Link>
      <Card>
        <CardHeader>
          <CardTitle>Privacy</CardTitle>
          <CardDescription>What Aniki stores when you use the product</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4 text-sm text-muted-foreground">
          <p>
            Aniki is a personal interview assistant. When you sign in with Google, we receive your
            verified email address and a Google account identifier. We use that to create or look up
            your Aniki account and to issue a session token stored in your browser.
          </p>
          <p>
            If you upload a resume, we store the file contents and derived text chunks so live
            answers can use your experience. Interview sessions may store transcripts and generated
            notes. We send resume excerpts, questions, and optional screen text to the language-model
            provider you have configured (for example OpenAI) to produce answers.
          </p>
          <p>
            We do not sell your data. Magic-link email sending is not enabled; Google is the current
            sign-in method. You can stop using Aniki by closing your account with the operator.
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
