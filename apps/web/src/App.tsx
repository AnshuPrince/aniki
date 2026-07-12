import { Navigate, Route, Routes } from "react-router-dom";
import { useAuth } from "./lib/auth";
import { DashboardLayout } from "./components/DashboardLayout";
import { LoginPage } from "./pages/LoginPage";
import { VerifyPage } from "./pages/VerifyPage";
import { DashboardPage } from "./pages/DashboardPage";
import { ResumesPage } from "./pages/ResumesPage";
import { SessionsPage } from "./pages/SessionsPage";
import { BillingPage } from "./pages/BillingPage";

function ProtectedRoute({ children }: { children: React.ReactNode }) {
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
      <Route path="/login" element={<LoginPage />} />
      <Route path="/auth/verify" element={<VerifyPage />} />
      <Route
        path="/"
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
    </Routes>
  );
}
