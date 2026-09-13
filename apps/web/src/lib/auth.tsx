import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import type { UserProfile } from "@aniki/shared";
import { api, TOKEN_KEY } from "./api";

interface AuthContextValue {
  user: UserProfile | null;
  loading: boolean;
  verify: (token: string) => Promise<void>;
  completeGoogle: (code: string, state: string) => Promise<void>;
  logout: () => void;
  refreshUser: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<UserProfile | null>(null);
  const [loading, setLoading] = useState(true);

  const refreshUser = useCallback(async () => {
    try {
      const profile = await api.me();
      setUser(profile);
    } catch {
      setUser(null);
      localStorage.removeItem(TOKEN_KEY);
    }
  }, []);

  useEffect(() => {
    const token = localStorage.getItem(TOKEN_KEY);
    if (token) {
      refreshUser().finally(() => setLoading(false));
    } else {
      setLoading(false);
    }
  }, [refreshUser]);

  const verify = useCallback(async (token: string) => {
    const response = await api.verifyToken(token);
    setUser(response.user);
  }, []);

  const completeGoogle = useCallback(async (code: string, state: string) => {
    const response = await api.oauthGoogleCallback(code, state);
    setUser(response.user);
  }, []);

  const logout = useCallback(() => {
    localStorage.removeItem(TOKEN_KEY);
    setUser(null);
  }, []);

  const value = useMemo(
    () => ({ user, loading, verify, completeGoogle, logout, refreshUser }),
    [user, loading, verify, completeGoogle, logout, refreshUser]
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
