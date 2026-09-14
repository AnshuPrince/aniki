import { Link } from "react-router-dom";
import { DesktopDownloads } from "../components/DesktopDownloads";
import { useAuth } from "../lib/auth";

const features = [
  {
    marker: "01",
    title: "Resume-aware answers",
    copy: "Your resume and instructions become context for concise, first-person answers while the call is happening.",
  },
  {
    marker: "02",
    title: "Two-channel listening",
    copy: "Microphone and system audio stay separate, so Aniki can distinguish the interviewer from you.",
  },
  {
    marker: "03",
    title: "Coding-screen context",
    copy: "On macOS, Screenshot reads the visible prompt with on-device OCR and includes it in the next answer.",
  },
  {
    marker: "04",
    title: "Notes after the call",
    copy: "Saved transcripts become a summary, key points, and action items when the session ends.",
  },
];

const faqs = [
  {
    question: "Does Aniki run in the browser?",
    answer:
      "The browser is your dashboard for resumes, sessions, and credits. Live listening and answers run in the desktop overlay.",
  },
  {
    question: "Which desktop platforms are supported?",
    answer:
      "The app builds for macOS and Windows. Screen OCR is currently macOS-only; Windows audio capture is available but still needs broader device testing.",
  },
  {
    question: "Can it help with coding interviews?",
    answer:
      "Yes. On macOS, Screenshot can read code or a prompt on screen and combine it with the spoken question. You can also paste context before the session.",
  },
  {
    question: "What language does it support?",
    answer:
      "The current product is configured for English. Multi-language selection is not part of this release.",
  },
  {
    question: "Is the overlay invisible on screen share?",
    answer:
      "Aniki uses platform capture protection and a non-activating overlay. Capture behavior can vary by OS and meeting app, so verify it before a call.",
  },
];

function BrandMark() {
  return (
    <span className="flex h-8 w-8 items-center justify-center rounded-full bg-primary text-sm font-black text-primary-foreground shadow-[0_0_30px_hsl(var(--primary)/0.35)]">
      A
    </span>
  );
}

function LivePreview() {
  return (
    <div className="relative mx-auto w-full max-w-xl" aria-label="Aniki live overlay preview">
      <div className="absolute -inset-12 -z-10 rounded-full bg-primary/10 blur-3xl" />
      <div className="overflow-hidden rounded-[1.75rem] border border-white/10 bg-black/70 shadow-2xl shadow-black/40 backdrop-blur-xl">
        <div className="flex items-center gap-2 border-b border-white/10 px-4 py-3">
          <BrandMark />
          <span className="text-sm font-semibold">Aniki</span>
          <span className="ml-auto rounded-full border border-white/10 px-3 py-1 text-[11px] text-muted-foreground">
            4.5 credits
          </span>
          <span className="flex h-6 w-6 items-center justify-center rounded-full bg-destructive/20 text-xs text-destructive">
            ×
          </span>
        </div>

        <div className="space-y-3 p-3 sm:p-4">
          <div className="flex flex-wrap items-center gap-2 rounded-2xl border border-white/10 bg-white/[0.04] p-3">
            <span className="flex items-center gap-1.5 rounded-full bg-primary/10 px-3 py-1.5 text-xs text-primary">
              <span className="h-1.5 w-1.5 rounded-full bg-primary" /> Listening
            </span>
            <span className="rounded-full bg-white/[0.06] px-3 py-1.5 text-xs">Mic on</span>
            <span className="rounded-full bg-white/[0.06] px-3 py-1.5 text-xs">System on</span>
            <button className="ml-auto rounded-full bg-primary px-4 py-1.5 text-xs font-semibold text-primary-foreground">
              End
            </button>
          </div>

          <div className="rounded-2xl border border-white/10 bg-white/[0.04] p-4">
            <p className="mb-3 text-[10px] font-semibold uppercase tracking-[0.22em] text-primary">
              Interviewer
            </p>
            <p className="text-base font-medium leading-relaxed sm:text-lg">
              Tell me about a time you improved the reliability of a production system.
            </p>
            <div className="mt-4 flex h-5 items-end gap-1" aria-hidden="true">
              {[30, 70, 42, 90, 55, 75, 35, 64, 48, 82, 38, 58].map((height, index) => (
                <span
                  key={`${height}-${index}`}
                  className="w-1 rounded-full bg-primary/70"
                  style={{ height: `${height}%` }}
                />
              ))}
            </div>
          </div>

          <div className="rounded-2xl border border-primary/20 bg-primary/[0.06] p-4">
            <div className="mb-3 flex items-center justify-between">
              <p className="text-[10px] font-semibold uppercase tracking-[0.22em] text-primary">
                Answer
              </p>
              <span className="text-[11px] text-muted-foreground">Resume context used</span>
            </div>
            <p className="text-sm leading-6 text-foreground/90">
              At my last company, I reduced API incidents by introducing request-level
              tracing and an explicit retry budget. I started by measuring the failure
              modes, then shipped the change behind a gradual rollout…
            </p>
          </div>

          <div className="grid grid-cols-3 gap-2">
            {["Answer", "Screenshot", "Chat"].map((action) => (
              <div
                key={action}
                className="rounded-xl border border-white/10 bg-white/[0.04] px-3 py-2 text-center text-xs font-medium"
              >
                {action}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

function SectionLabel({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <p className="mb-4 text-xs font-bold uppercase tracking-[0.28em] text-primary">{children}</p>
  );
}

export function LandingPage() {
  const { user } = useAuth();

  return (
    <div className="min-h-screen overflow-hidden bg-background text-foreground">
      <a
        href="#main"
        className="sr-only z-50 rounded-md bg-primary px-4 py-2 text-primary-foreground focus:not-sr-only focus:fixed focus:left-4 focus:top-4"
      >
        Skip to content
      </a>

      <header className="fixed inset-x-0 top-0 z-40 border-b border-white/[0.06] bg-background/80 backdrop-blur-xl">
        <div className="mx-auto flex h-16 max-w-7xl items-center px-5 sm:px-8">
          <Link to="/" className="flex items-center gap-2.5 font-bold">
            <BrandMark />
            Aniki
          </Link>
          <nav className="ml-auto hidden items-center gap-7 text-sm text-muted-foreground md:flex">
            <a href="#features" className="transition hover:text-foreground">Features</a>
            <a href="#privacy" className="transition hover:text-foreground">Privacy</a>
            <a href="#pricing" className="transition hover:text-foreground">Pricing</a>
            <a href="#faq" className="transition hover:text-foreground">FAQ</a>
          </nav>
          <div className="ml-auto flex items-center gap-3 md:ml-8">
            {user ? <DesktopDownloads /> : null}
            <Link
              to={user ? "/app" : "/login"}
              className="rounded-full bg-primary px-4 py-2 text-sm font-bold text-primary-foreground transition hover:brightness-110"
            >
              {user ? "Open app" : "Get started"}
            </Link>
          </div>
        </div>
      </header>

      <main id="main">
        <section className="relative px-5 pb-24 pt-36 sm:px-8 sm:pb-32 sm:pt-44">
          <div className="absolute left-1/2 top-0 -z-10 h-[640px] w-[900px] -translate-x-1/2 rounded-full bg-[radial-gradient(circle,hsl(var(--primary)/0.12),transparent_65%)]" />
          <div className="mx-auto grid max-w-7xl items-center gap-16 lg:grid-cols-[1fr_0.92fr]">
            <div>
              <div className="mb-6 inline-flex items-center gap-2 rounded-full border border-primary/20 bg-primary/[0.06] px-3 py-1.5 text-xs font-semibold text-primary">
                <span className="h-1.5 w-1.5 rounded-full bg-primary" />
                <span>Real-time interview copilot</span>
              </div>
              <h1 className="max-w-3xl text-5xl font-black leading-[0.98] tracking-[-0.045em] sm:text-6xl lg:text-7xl">
                <span className="block">Your experience,</span>
                <span className="block text-primary">ready when asked.</span>
              </h1>
              <p className="mt-7 max-w-xl text-lg leading-8 text-muted-foreground">
                Aniki listens to the call, finds the relevant part of your resume, and
                drafts a natural answer while the conversation is still moving.
              </p>
              <div className="mt-9 flex flex-col gap-3 sm:flex-row">
                <Link
                  to={user ? "/app" : "/login"}
                  className="rounded-full bg-primary px-7 py-3.5 text-center text-sm font-bold text-primary-foreground transition hover:brightness-110"
                >
                  {user ? "Open your dashboard" : "Start with 5 free credits"}
                </Link>
                <a
                  href="#how-it-works"
                  className="rounded-full border border-white/10 bg-white/[0.04] px-7 py-3.5 text-center text-sm font-semibold transition hover:bg-white/[0.08]"
                >
                  See how it works
                </a>
              </div>
              <p className="mt-4 text-xs text-muted-foreground">
                No credit card. Use only where AI assistance is permitted.
              </p>
            </div>
            <LivePreview />
          </div>
        </section>

        <section id="how-it-works" className="border-y border-white/[0.06] bg-white/[0.02] px-5 py-24 sm:px-8">
          <div className="mx-auto max-w-7xl">
            <SectionLabel>Before, during, after</SectionLabel>
            <h2 className="max-w-2xl text-3xl font-bold tracking-tight sm:text-5xl">
              One quiet workflow for the whole call.
            </h2>
            <div className="mt-14 grid gap-px overflow-hidden rounded-3xl border border-white/10 bg-white/10 md:grid-cols-3">
              {[
                ["01", "Prepare", "Upload a resume and add the role, company, and any instructions you want Aniki to follow."],
                ["02", "Listen", "Open the desktop overlay. Aniki separates microphone and system audio and detects interview questions."],
                ["03", "Review", "End the session to save the transcript and generate a summary, key points, and next actions."],
              ].map(([number, title, copy]) => (
                <article key={number} className="bg-card p-7 sm:p-9">
                  <span className="text-sm font-black text-primary">{number}</span>
                  <h3 className="mt-8 text-2xl font-bold">{title}</h3>
                  <p className="mt-3 text-sm leading-6 text-muted-foreground">{copy}</p>
                </article>
              ))}
            </div>
          </div>
        </section>

        <section id="features" className="px-5 py-24 sm:px-8 sm:py-32">
          <div className="mx-auto max-w-7xl">
            <div className="grid gap-12 lg:grid-cols-[0.7fr_1.3fr]">
              <div>
                <SectionLabel>Mid-conversation</SectionLabel>
                <h2 className="text-3xl font-bold tracking-tight sm:text-5xl">
                  The right context, without leaving the call.
                </h2>
                <p className="mt-5 max-w-md leading-7 text-muted-foreground">
                  A small desktop overlay keeps listening status, questions, and answers
                  together. Collapse it to a pebble whenever you need more space.
                </p>
              </div>
              <div className="grid gap-4 sm:grid-cols-2">
                {features.map((feature) => (
                  <article
                    key={feature.marker}
                    className="rounded-3xl border border-white/[0.08] bg-card/70 p-7 transition hover:border-primary/25 hover:bg-card"
                  >
                    <span className="text-xs font-black text-primary">{feature.marker}</span>
                    <h3 className="mt-10 text-xl font-bold">{feature.title}</h3>
                    <p className="mt-3 text-sm leading-6 text-muted-foreground">{feature.copy}</p>
                  </article>
                ))}
              </div>
            </div>
          </div>
        </section>

        <section id="privacy" className="px-5 py-24 sm:px-8">
          <div className="mx-auto max-w-7xl overflow-hidden rounded-[2rem] border border-primary/20 bg-primary/[0.055] p-8 sm:p-14 lg:p-20">
            <div className="grid gap-12 lg:grid-cols-2 lg:items-center">
              <div>
                <SectionLabel>Privacy by design</SectionLabel>
                <h2 className="text-3xl font-bold tracking-tight sm:text-5xl">
                  Present when you need it. Out of the way when you don’t.
                </h2>
                <p className="mt-5 max-w-xl leading-7 text-muted-foreground">
                  Aniki has no Dock icon on macOS, supports click-through, and uses
                  platform capture protection. The overlay can collapse to a one-centimeter
                  pebble or hide completely from its menu.
                </p>
                <p className="mt-4 max-w-xl text-sm leading-6 text-muted-foreground">
                  Capture behavior varies by OS and conferencing app. Test your setup first;
                  we do not claim universal undetectability.
                </p>
              </div>
              <div className="grid grid-cols-2 gap-3">
                {["No Dock icon", "Click-through", "Capture protection", "Pebble mode"].map((item) => (
                  <div key={item} className="rounded-2xl border border-white/10 bg-black/20 p-5">
                    <span className="mb-8 block h-2 w-2 rounded-full bg-primary" />
                    <p className="text-sm font-semibold">{item}</p>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </section>

        <section id="pricing" className="px-5 py-24 sm:px-8 sm:py-32">
          <div className="mx-auto max-w-5xl text-center">
            <SectionLabel>Simple credits</SectionLabel>
            <h2 className="text-3xl font-bold tracking-tight sm:text-5xl">
              Start free. Pay only when you’re ready.
            </h2>
            <p className="mx-auto mt-5 max-w-2xl leading-7 text-muted-foreground">
              New accounts receive five credits. Starting a call session uses 0.5 credit.
              Credit purchasing and subscriptions are coming later.
            </p>
            <div className="mx-auto mt-12 max-w-md rounded-[2rem] border border-primary/25 bg-card p-8 text-left shadow-2xl shadow-primary/5">
              <p className="text-sm font-bold text-primary">Free account</p>
              <div className="mt-5 flex items-baseline gap-2">
                <span className="text-5xl font-black">5</span>
                <span className="text-muted-foreground">credits included</span>
              </div>
              <ul className="mt-8 space-y-3 text-sm text-muted-foreground">
                {["Live desktop overlay", "Resume-aware answers", "Transcripts and AI notes", "macOS screen OCR"].map((item) => (
                  <li key={item} className="flex gap-3">
                    <span className="text-primary">✓</span> {item}
                  </li>
                ))}
              </ul>
              <Link
                to="/login"
                className="mt-9 block rounded-full bg-primary px-6 py-3 text-center text-sm font-bold text-primary-foreground"
              >
                Create free account
              </Link>
            </div>
          </div>
        </section>

        <section id="faq" className="border-t border-white/[0.06] px-5 py-24 sm:px-8">
          <div className="mx-auto grid max-w-5xl gap-12 lg:grid-cols-[0.55fr_1fr]">
            <div>
              <SectionLabel>FAQ</SectionLabel>
              <h2 className="text-3xl font-bold tracking-tight">Questions, answered.</h2>
            </div>
            <div className="divide-y divide-white/[0.08] border-y border-white/[0.08]">
              {faqs.map((item) => (
                <details key={item.question} className="group py-5">
                  <summary className="flex cursor-pointer list-none items-center justify-between gap-6 font-semibold">
                    {item.question}
                    <span className="text-xl text-primary transition group-open:rotate-45">+</span>
                  </summary>
                  <p className="max-w-2xl pt-3 text-sm leading-6 text-muted-foreground">{item.answer}</p>
                </details>
              ))}
            </div>
          </div>
        </section>

        <section className="px-5 pb-24 pt-10 sm:px-8">
          <div className="mx-auto max-w-7xl rounded-[2rem] bg-primary px-7 py-14 text-center text-primary-foreground sm:py-20">
            <h2 className="text-3xl font-black tracking-tight sm:text-5xl">
              Be ready for the next question.
            </h2>
            <p className="mx-auto mt-4 max-w-xl text-sm opacity-75 sm:text-base">
              Bring your resume, open the overlay, and keep the conversation moving.
            </p>
            <Link
              to="/login"
              className="mt-8 inline-flex rounded-full bg-primary-foreground px-7 py-3.5 text-sm font-bold text-primary"
            >
              Try Aniki free
            </Link>
          </div>
        </section>
      </main>

      <footer className="border-t border-white/[0.06] px-5 py-10 sm:px-8">
        <div className="mx-auto flex max-w-7xl flex-col gap-6 text-sm text-muted-foreground sm:flex-row sm:items-center">
          <div className="flex items-center gap-2.5 text-foreground">
            <BrandMark />
            <span className="font-bold">Aniki</span>
          </div>
          <p className="sm:ml-4">Privacy-focused real-time call assistance.</p>
          <nav className="flex flex-wrap gap-5 sm:ml-auto">
            <a href="#features" className="hover:text-foreground">Features</a>
            <Link to="/privacy" className="hover:text-foreground">Privacy</Link>
            <a href="#pricing" className="hover:text-foreground">Pricing</a>
            <Link to="/login" className="hover:text-foreground">Sign in</Link>
          </nav>
        </div>
      </footer>
    </div>
  );
}
