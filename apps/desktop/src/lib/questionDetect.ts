const WH_STARTERS = [
  "what ",
  "why ",
  "how ",
  "when ",
  "where ",
  "who ",
  "which ",
  "can you ",
  "could you ",
  "would you ",
  "tell me ",
  "describe ",
];

export function heuristicIsQuestion(text: string): boolean {
  const trimmed = text.trim();
  if (trimmed.length < 6) return false;
  if (trimmed.endsWith("?")) return true;
  const lower = trimmed.toLowerCase();
  return WH_STARTERS.some((s) => lower.startsWith(s));
}

export function shouldTriggerAnswer(
  speaker: string,
  text: string,
  isFinal: boolean,
  lastHash: string | null
): { trigger: boolean; hash: string | null } {
  if (!isFinal || speaker !== "interviewer") {
    return { trigger: false, hash: lastHash };
  }
  if (!heuristicIsQuestion(text)) {
    return { trigger: false, hash: lastHash };
  }
  const hash = simpleHash(text);
  if (lastHash === hash) {
    return { trigger: false, hash: lastHash };
  }
  return { trigger: true, hash };
}

function simpleHash(text: string): string {
  return text.trim().toLowerCase().replace(/\s+/g, " ");
}

// Debounce map for 1.5s silence after interviewer speech
const debounceTimers = new Map<string, ReturnType<typeof setTimeout>>();

export function debouncedQuestionCheck(
  segmentId: string,
  speaker: string,
  text: string,
  isFinal: boolean,
  onQuestion: (question: string) => void | Promise<void>,
  delayMs = 1500
) {
  if (!isFinal || speaker !== "interviewer") return;

  const existing = debounceTimers.get(segmentId);
  if (existing) clearTimeout(existing);

  debounceTimers.set(
    segmentId,
    setTimeout(() => {
      if (heuristicIsQuestion(text)) {
        void onQuestion(text);
      }
      debounceTimers.delete(segmentId);
    }, delayMs)
  );
}
