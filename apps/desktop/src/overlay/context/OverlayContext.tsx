import type { CreateSessionResponse, LlmModel, Resume, Session, UserProfile } from "@aniki/shared";
import { STT_JWT_REFRESH_INTERVAL_MS } from "@aniki/shared";
import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { api, clearAccessToken, restoreAccessToken, setAccessToken } from "../../lib/api";
import { debouncedQuestionCheck } from "../../lib/questionDetect";
import { useCollapse } from "../hooks/useCollapse";
import { setLiveSize, setShellSize } from "../hooks/useWindowSize";
import type { OverlayScreen, TranscriptLine } from "../types";

interface OverlayContextValue {
  screen: OverlayScreen;
  user: UserProfile | null;
  collapsed: boolean;
  collapse: () => Promise<void>;
  expand: () => Promise<void>;
  login: (verifyToken: string) => Promise<void>;
  loginWithAccessToken: (accessToken: string) => Promise<void>;
  logout: () => void;
  session: CreateSessionResponse | null;
  transcript: TranscriptLine[];
  answer: string;
  micLevel: number;
  micActive: boolean;
  systemActive: boolean;
  devSttMode: boolean;
  autoAnswer: boolean;
  setAutoAnswer: (v: boolean) => void;
  clickThrough: boolean;
  stealthWarning: string | null;
  starting: boolean;
  startSession: (params: StartSessionParams) => Promise<void>;
  joinSession: (sessionId: string) => Promise<void>;
  endSession: () => Promise<void>;
  streamAnswer: (question: string, withOcr?: boolean) => Promise<void>;
  lastInterviewerQuestion: string;
  clearQuestion: () => void;
}

export interface StartSessionParams {
  resumeId?: string;
  model: LlmModel;
  enableOcr: boolean;
  extraContext?: string;
}

const OverlayContext = createContext<OverlayContextValue | null>(null);

export function useOverlay() {
  const ctx = useContext(OverlayContext);
  if (!ctx) throw new Error("useOverlay must be used within OverlayProvider");
  return ctx;
}

export function OverlayProvider({ children }: { children: ReactNode }) {
  const { collapsed, collapse, expand } = useCollapse();
  const [screen, setScreen] = useState<OverlayScreen>("login");
  const [user, setUser] = useState<UserProfile | null>(null);
  const [session, setSession] = useState<CreateSessionResponse | null>(null);
  const [transcript, setTranscript] = useState<TranscriptLine[]>([]);
  const [answer, setAnswer] = useState("");
  const [micLevel, setMicLevel] = useState(0);
  const [micActive, setMicActive] = useState(false);
  const [systemActive, setSystemActive] = useState(false);
  const [devSttMode, setDevSttMode] = useState(false);
  const [autoAnswer, setAutoAnswer] = useState(true);
  const [clickThrough, setClickThrough] = useState(false);
  const [stealthWarning, setStealthWarning] = useState<string | null>(null);
  const [starting, setStarting] = useState(false);
  const [lastInterviewerQuestion, setLastInterviewerQuestion] = useState("");

  const refreshTimer = useRef<ReturnType<typeof setInterval> | null>(null);
  const lastQuestionHash = useRef<string | null>(null);
  const finalSegments = useRef<TranscriptLine[]>([]);
  const autoAnswerRef = useRef(autoAnswer);
  autoAnswerRef.current = autoAnswer;

  useEffect(() => {
    restoreAccessToken()
      .then((token) => {
        if (!token) return;
        return api.me();
      })
      .then((u) => {
        if (!u) return;
        setUser(u);
        setScreen("shell");
      })
      .catch(() => {
        void clearAccessToken();
      });
  }, []);

  useEffect(() => {
    invoke<{ capture_exclusion_degraded: boolean; degradation_reason?: string }>(
      "get_stealth_status",
    ).then((s) => {
      if (s.capture_exclusion_degraded && s.degradation_reason) {
        setStealthWarning(s.degradation_reason);
      }
    });
  }, []);

  useEffect(() => {
    invoke<boolean>("get_click_through")
      .then(setClickThrough)
      .catch(() => {});
    const unlisten = listen<boolean>("click-through-changed", (e) => setClickThrough(e.payload));
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  useEffect(() => {
    const unlisten = listen<string>("mic-level", (event) => {
      try {
        const data = JSON.parse(event.payload) as {
          level: number;
          active: boolean;
          systemActive?: boolean;
        };
        setMicLevel(data.level);
        setMicActive(data.active);
        if (typeof data.systemActive === "boolean") setSystemActive(data.systemActive);
      } catch {
        // ignore
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const streamAnswer = useCallback(
    async (question: string, withOcr = false) => {
      if (!session) return;
      setAnswer("");
      void emit("answer-update", "");
      try {
        let ocrContext: string | undefined;
        if (withOcr || session.session.enable_screen_ocr) {
          ocrContext = await invoke<string>("capture_screen_ocr");
        }
        const transcriptContext = finalSegments.current
          .slice(-6)
          .map((t) => `[${t.speaker}] ${t.text}`)
          .join("\n");

        for await (const chunk of api.streamAnswer(
          session.session.id,
          question,
          transcriptContext,
          ocrContext,
        )) {
          if (chunk) {
            setAnswer((prev) => {
              const next = prev + chunk;
              void emit("answer-update", next);
              return next;
            });
          }
        }
      } catch (e) {
        const message = e instanceof Error ? e.message : "Failed to get answer";
        setAnswer(message);
        void emit("answer-update", message);
      }
    },
    [session],
  );

  useEffect(() => {
    const unlisten = listen<string>("transcript-update", (event) => {
      try {
        const data = JSON.parse(event.payload) as TranscriptLine;
        setTranscript((prev) => {
          const existing = prev.findIndex((t) => t.id === data.id);
          if (existing >= 0) {
            const next = [...prev];
            next[existing] = data;
            return next;
          }
          return [...prev, data];
        });

        if (data.speaker === "interviewer" && data.text.trim()) {
          setLastInterviewerQuestion(data.text);
        }

        if (data.isFinal) {
          finalSegments.current = [...finalSegments.current, data].slice(-50);
        }

        if (!autoAnswerRef.current) return;

        debouncedQuestionCheck(data.id, data.speaker, data.text, data.isFinal, async (question) => {
          const hash = question.trim().toLowerCase();
          if (lastQuestionHash.current === hash) return;
          try {
            const { is_question } = await api.confirmQuestion(question);
            if (!is_question) return;
          } catch {
            // heuristic fallback
          }
          lastQuestionHash.current = hash;
          streamAnswer(question);
        });
      } catch {
        // ignore
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [streamAnswer]);

  const login = useCallback(async (verifyToken: string) => {
    const response = await api.verifyToken(verifyToken);
    await setAccessToken(response.access_token);
    setUser(response.user);
    setScreen("shell");
    await setShellSize();
  }, []);

  /// The web dashboard hands out a session JWT, not a magic-link token.
  const loginWithAccessToken = useCallback(async (accessToken: string) => {
    await setAccessToken(accessToken);
    try {
      const profile = await api.me();
      setUser(profile);
      setScreen("shell");
      await setShellSize();
    } catch (e) {
      await clearAccessToken();
      throw e instanceof Error && e.message
        ? new Error("That access token was rejected. Copy a fresh one from the web dashboard.")
        : e;
    }
  }, []);

  const logout = useCallback(() => {
    void clearAccessToken();
    setUser(null);
    setScreen("login");
    setSession(null);
  }, []);

  const beginLiveSession = useCallback(async (response: CreateSessionResponse) => {
    setSession(response);
    setTranscript([]);
    setAnswer("");
    finalSegments.current = [];
    lastQuestionHash.current = null;

    const audioStatus = await invoke<{
      mic_active: boolean;
      dev_stt_mode: boolean;
    }>("start_audio_session", {
      sttJwt: response.stt_jwt,
      sttEndpoint: response.stt_endpoint,
    });

    setMicActive(audioStatus.mic_active);
    setDevSttMode(audioStatus.dev_stt_mode);

    if (refreshTimer.current) clearInterval(refreshTimer.current);
    refreshTimer.current = setInterval(async () => {
      try {
        const refreshed = await api.refreshSttJwt(response.session.id);
        await invoke("refresh_stt_jwt", { jwt: refreshed.jwt });
      } catch (e) {
        console.error("STT JWT refresh failed", e);
      }
    }, STT_JWT_REFRESH_INTERVAL_MS);

    setScreen("live");
    await setLiveSize();
  }, []);

  const startSession = useCallback(
    async (params: StartSessionParams) => {
      setStarting(true);
      try {
        const response = await api.createSession({
          resume_id: params.resumeId || undefined,
          model: params.model,
          enable_screen_ocr: params.enableOcr,
          extra_context: params.extraContext || undefined,
        });
        await beginLiveSession(response);
      } finally {
        setStarting(false);
      }
    },
    [beginLiveSession],
  );

  const joinSession = useCallback(
    async (sessionId: string) => {
      setStarting(true);
      try {
        const detail = await api.getSession(sessionId.trim());
        if (detail.session.status !== "active") {
          throw new Error("Only an active session can be joined.");
        }
        const stt = await api.refreshSttJwt(detail.session.id);
        await beginLiveSession({
          session: detail.session,
          stt_jwt: stt.jwt,
          stt_endpoint: stt.endpoint,
          stt_expires_at: stt.expires_at,
        });
      } finally {
        setStarting(false);
      }
    },
    [beginLiveSession],
  );

  const endSession = useCallback(async () => {
    if (refreshTimer.current) {
      clearInterval(refreshTimer.current);
      refreshTimer.current = null;
    }

    if (session) {
      const segments = finalSegments.current.map((s) => ({
        id: s.id,
        speaker: s.speaker,
        text: s.text,
        is_final: s.isFinal,
      }));
      await api.appendTranscript(session.session.id, segments).catch(console.error);
      await api.finalizeSession(session.session.id).catch(console.error);
    }

    await invoke("stop_audio_session").catch(console.error);
    setSession(null);
    setMicLevel(0);
    setMicActive(false);
    setDevSttMode(false);
    setTranscript([]);
    setAnswer("");
    setLastInterviewerQuestion("");
    finalSegments.current = [];
    setScreen("shell");
    await setShellSize();
  }, [session]);

  const clearQuestion = useCallback(() => {
    setLastInterviewerQuestion("");
    lastQuestionHash.current = null;
  }, []);

  return (
    <OverlayContext.Provider
      value={{
        screen,
        user,
        collapsed,
        collapse,
        expand,
        login,
        loginWithAccessToken,
        logout,
        session,
        transcript,
        answer,
        micLevel,
        micActive,
        systemActive,
        devSttMode,
        autoAnswer,
        setAutoAnswer,
        clickThrough,
        stealthWarning,
        starting,
        startSession,
        joinSession,
        endSession,
        streamAnswer,
        lastInterviewerQuestion,
        clearQuestion,
      }}
    >
      {children}
    </OverlayContext.Provider>
  );
}

export type { Resume, Session };
