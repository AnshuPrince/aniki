import { Navigate, Route, Routes } from "react-router-dom";
import { useAuth } from "./lib/auth";
import { DashboardLayout } from "./components/DashboardLayout";
import { LoginPage } from "./pages/LoginPage";
import { VerifyPage } from "./pages/VerifyPage";
import { DashboardPage } from "./pages/DashboardPage";
import { ResumesPage } from "./pages/ResumesPage";
import { SessionsPage } from "./pages/SessionsPage";
import { BillingPage } from "./pages/BillingPage";
import { LandingPage } from "./pages/LandingPage";
import { OAuthCallbackPage } from "./pages/OAuthCallbackPage";
import { PrivacyPage } from "./pages/PrivacyPage";

function ProtectedRoute({ children }: Readonly<{ children: React.ReactNode }>) {
  const { user, loading } = useAuth();
  if (loading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }
  if (!user) return <Navigate to="/login" replace />;
  return <>{children}</>;
}

export function App() {
  return (
    <Routes>
      <Route path="/" element={<LandingPage />} />
      <Route path="/login" element={<LoginPage />} />
      <Route path="/privacy" element={<PrivacyPage />} />
      <Route path="/auth/verify" element={<VerifyPage />} />
      <Route path="/auth/oauth/callback" element={<OAuthCallbackPage />} />
      <Route
        path="/app"
        element={
          <ProtectedRoute>
            <DashboardLayout />
          </ProtectedRoute>
        }
      >
        <Route index element={<DashboardPage />} />
        <Route path="resumes" element={<ResumesPage />} />
        <Route path="sessions" element={<SessionsPage />} />
        <Route path="billing" element={<BillingPage />} />
      </Route>
      <Route path="/resumes" element={<Navigate to="/app/resumes" replace />} />
      <Route path="/sessions" element={<Navigate to="/app/sessions" replace />} />
      <Route path="/billing" element={<Navigate to="/app/billing" replace />} />
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  );
}
