import { useEffect, useState } from "react";
import type { CreditLedgerEntry } from "@aniki/shared";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@aniki/ui";
import { api } from "../lib/api";

export function BillingPage() {
  const [balance, setBalance] = useState(0);
  const [entries, setEntries] = useState<CreditLedgerEntry[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api
      .getCredits()
      .then((data) => {
        setBalance(data.balance);
        setEntries(data.entries);
      })
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold">Billing</h1>
        <p className="text-muted-foreground">Credits and payment history (Stripe integration in Stage B)</p>
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Credit balance</CardTitle>
            <CardDescription>0.5 credits per 30-minute session</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-4xl font-bold text-primary">
              {loading ? "—" : balance.toFixed(1)}
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Buy credits</CardTitle>
            <CardDescription>Payment gateway coming in Stage B</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              $9.99 for 10 credits · $29/mo unlimited (planned)
            </p>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Transaction history</CardTitle>
        </CardHeader>
        <CardContent>
          {loading ? (
            <p className="text-sm text-muted-foreground">Loading...</p>
          ) : entries.length === 0 ? (
            <p className="text-sm text-muted-foreground">No transactions yet.</p>
          ) : (
            <ul className="divide-y divide-border">
              {entries.map((entry) => (
                <li key={entry.id} className="flex justify-between py-2 text-sm">
                  <span>{entry.reason}</span>
                  <span className={entry.delta >= 0 ? "text-primary" : "text-destructive"}>
                    {entry.delta >= 0 ? "+" : ""}
                    {entry.delta.toFixed(1)}
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
