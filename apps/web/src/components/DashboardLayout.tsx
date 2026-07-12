import { NavLink, Outlet } from "react-router-dom";
import { Button } from "@aniki/ui";
import { useAuth } from "../lib/auth";
import { cn } from "@aniki/ui";

const navItems = [
  { to: "/", label: "Dashboard" },
  { to: "/resumes", label: "Resumes" },
  { to: "/sessions", label: "Sessions" },
  { to: "/billing", label: "Billing" },
];

export function DashboardLayout() {
  const { user, logout } = useAuth();

  return (
    <div className="min-h-screen">
      <header className="border-b border-border">
        <div className="mx-auto flex max-w-6xl items-center justify-between px-6 py-4">
          <div className="flex items-center gap-8">
            <span className="text-xl font-bold text-primary">Aniki</span>
            <nav className="flex gap-1">
              {navItems.map((item) => (
                <NavLink
                  key={item.to}
                  to={item.to}
                  end={item.to === "/"}
                  className={({ isActive }) =>
                    cn(
                      "rounded-md px-3 py-2 text-sm font-medium transition-colors",
                      isActive
                        ? "bg-secondary text-foreground"
                        : "text-muted-foreground hover:text-foreground"
                    )
                  }
                >
                  {item.label}
                </NavLink>
              ))}
            </nav>
          </div>
          <div className="flex items-center gap-4">
            <span className="text-sm text-muted-foreground">
              {user?.email} · {user?.credits?.toFixed(1)} credits
            </span>
            <Button variant="outline" size="sm" onClick={logout}>
              Sign out
            </Button>
          </div>
        </div>
      </header>
      <main className="mx-auto max-w-6xl px-6 py-8">
        <Outlet />
      </main>
    </div>
  );
}
