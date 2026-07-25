//! App-level raw CSS: keyframes and the handful of effects that are cheaper to
//! express as a stylesheet than as per-node signals (grain, marquee, masks).
//!
//! Everything here is decoration. All *state* still flows through signals — see
//! [`crate::fx`] for the pointer-reactive parts.

pub const APP_STYLES: &str = r#"
/* ------------------------------------------------------------------ */
/* keyframes                                                           */
/* ------------------------------------------------------------------ */

@keyframes dwind-cursor-blink {
    0%, 49% { opacity: 1; }
    50%, 100% { opacity: 0; }
}

@keyframes dwind-fade-up {
    from { opacity: 0; transform: translateY(14px); }
    to   { opacity: 1; transform: translateY(0); }
}

@keyframes dwind-glow-drift {
    0%, 100% { transform: translate(0, 0) scale(1); }
    50%      { transform: translate(4%, -6%) scale(1.08); }
}

/* slow, organic drift for the aurora blobs */
@keyframes dwind-aurora-a {
    0%, 100% { transform: translate3d(0, 0, 0) scale(1); }
    33%      { transform: translate3d(6%, -8%, 0) scale(1.15); }
    66%      { transform: translate3d(-5%, 5%, 0) scale(0.95); }
}

@keyframes dwind-aurora-b {
    0%, 100% { transform: translate3d(0, 0, 0) scale(1.05); }
    50%      { transform: translate3d(-8%, 6%, 0) scale(0.9); }
}

/* the sheen that sweeps across gradient headlines */
@keyframes dwind-sheen {
    0%   { background-position: 0% 50%; }
    100% { background-position: 200% 50%; }
}

/* word-by-word headline reveal */
@keyframes dwind-word-in {
    from { opacity: 0; transform: translateY(0.7em) rotate(2deg); }
    to   { opacity: 1; transform: translateY(0) rotate(0deg); }
}

/* infinite utility-class ticker */
@keyframes dwind-marquee {
    from { transform: translate3d(0, 0, 0); }
    to   { transform: translate3d(-50%, 0, 0); }
}

/* command palette entrance */
@keyframes dwind-palette-in {
    from { opacity: 0; transform: translateY(-12px) scale(0.98); }
    to   { opacity: 1; transform: translateY(0) scale(1); }
}

@keyframes dwind-scrim-in {
    from { opacity: 0; }
    to   { opacity: 1; }
}

/* route change transition */
@keyframes dwind-route-in {
    from { opacity: 0; transform: translateY(10px); }
    to   { opacity: 1; transform: translateY(0); }
}

@keyframes dwind-pulse-ring {
    0%   { box-shadow: 0 0 0 0 rgba(213, 182, 95, 0.35); }
    70%  { box-shadow: 0 0 0 12px rgba(213, 182, 95, 0); }
    100% { box-shadow: 0 0 0 0 rgba(213, 182, 95, 0); }
}

/* ------------------------------------------------------------------ */
/* scroll-triggered progressive reveal (see reveal.rs)                  */
/* ------------------------------------------------------------------ */

.reveal-section > * {
    opacity: 0;
    transform: translateY(26px);
    transition:
        opacity 650ms cubic-bezier(0.16, 1, 0.3, 1),
        transform 650ms cubic-bezier(0.16, 1, 0.3, 1);
}

.reveal-section > *:nth-child(2) { transition-delay: 70ms; }
.reveal-section > *:nth-child(3) { transition-delay: 140ms; }
.reveal-section > *:nth-child(4) { transition-delay: 210ms; }
.reveal-section > *:nth-child(5) { transition-delay: 280ms; }

.reveal-section.reveal-in > * {
    opacity: 1;
    transform: translateY(0);
}

/* ------------------------------------------------------------------ */
/* film grain — one fixed overlay for the whole app                     */
/* ------------------------------------------------------------------ */

.dw-grain {
    position: fixed;
    inset: 0;
    z-index: 9998;
    pointer-events: none;
    opacity: 0.22;
    mix-blend-mode: overlay;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='140' height='140'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.85' numOctaves='3' stitchTiles='stitch'/%3E%3CfeColorMatrix type='saturate' values='0'/%3E%3C/filter%3E%3Crect width='140' height='140' filter='url(%23n)' opacity='0.5'/%3E%3C/svg%3E");
}

/* ------------------------------------------------------------------ */
/* gradient text with a slow sheen sweep                                */
/* ------------------------------------------------------------------ */

.dw-sheen {
    background-image: linear-gradient(
        100deg,
        #F0E2B6 0%,
        #D5B65F 18%,
        #FFF8E2 30%,
        #D5B65F 42%,
        #A88735 60%,
        #D5B65F 100%
    );
    background-size: 200% auto;
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
    animation: dwind-sheen 7s linear infinite;
}

/* ------------------------------------------------------------------ */
/* kinetic headline: each word rides in on its own delay                */
/* ------------------------------------------------------------------ */

.dw-word {
    display: inline-block;
    animation: dwind-word-in 900ms cubic-bezier(0.16, 1, 0.3, 1) both;
}

/* ------------------------------------------------------------------ */
/* pointer spotlight cards (coordinates are written by fx.rs signals)   */
/* ------------------------------------------------------------------ */

.dw-spot {
    position: relative;
    isolation: isolate;
    transition: transform 400ms cubic-bezier(0.16, 1, 0.3, 1),
                border-color 300ms ease;
}

/* the glow itself — a radial gradient parked at --sx/--sy */
.dw-spot::before {
    content: "";
    position: absolute;
    inset: 0;
    z-index: -1;
    border-radius: inherit;
    opacity: 0;
    transition: opacity 320ms ease;
    background: radial-gradient(
        22rem circle at var(--sx, 50%) var(--sy, 50%),
        rgba(213, 182, 95, 0.13),
        transparent 62%
    );
}

.dw-spot[data-hot="1"]::before { opacity: 1; }

/* a thin lit edge that tracks the cursor too */
.dw-spot::after {
    content: "";
    position: absolute;
    inset: 0;
    z-index: -1;
    border-radius: inherit;
    padding: 1px;
    opacity: 0;
    transition: opacity 320ms ease;
    background: radial-gradient(
        16rem circle at var(--sx, 50%) var(--sy, 50%),
        rgba(213, 182, 95, 0.55),
        transparent 55%
    );
    -webkit-mask:
        linear-gradient(#000 0 0) content-box,
        linear-gradient(#000 0 0);
    -webkit-mask-composite: xor;
    mask:
        linear-gradient(#000 0 0) content-box,
        linear-gradient(#000 0 0);
    mask-composite: exclude;
}

.dw-spot[data-hot="1"]::after { opacity: 1; }

/* ------------------------------------------------------------------ */
/* marquee                                                              */
/* ------------------------------------------------------------------ */

.dw-marquee {
    -webkit-mask-image: linear-gradient(90deg, transparent, #000 12%, #000 88%, transparent);
    mask-image: linear-gradient(90deg, transparent, #000 12%, #000 88%, transparent);
}

.dw-marquee-track {
    display: flex;
    width: max-content;
    animation: dwind-marquee 42s linear infinite;
}

.dw-marquee:hover .dw-marquee-track { animation-play-state: paused; }

/* ------------------------------------------------------------------ */
/* command palette                                                      */
/* ------------------------------------------------------------------ */

.dw-palette-scrim {
    animation: dwind-scrim-in 180ms ease-out both;
    backdrop-filter: blur(6px) saturate(0.7);
}

.dw-palette {
    animation: dwind-palette-in 240ms cubic-bezier(0.16, 1, 0.3, 1) both;
    backdrop-filter: blur(20px) saturate(1.4);
    box-shadow:
        0 32px 80px -12px rgba(0, 0, 0, 0.85),
        0 0 0 1px rgba(213, 182, 95, 0.14),
        inset 0 1px 0 0 rgba(255, 255, 255, 0.05);
}

.dw-palette-input {
    background: transparent;
    border: none;
    outline: none;
    color: #F5F5F6;
    width: 100%;
    font-size: 1.05rem;
    font-family: inherit;
}

.dw-palette-input::placeholder { color: #55555F; }

/* ------------------------------------------------------------------ */
/* glass surfaces                                                       */
/* ------------------------------------------------------------------ */

.dw-glass {
    background: linear-gradient(
        160deg,
        rgba(28, 28, 33, 0.72) 0%,
        rgba(14, 14, 17, 0.62) 100%
    );
    backdrop-filter: blur(14px) saturate(1.2);
    box-shadow: inset 0 1px 0 0 rgba(255, 255, 255, 0.045);
}

/* ------------------------------------------------------------------ */
/* route transition                                                     */
/* ------------------------------------------------------------------ */

.dw-route {
    animation: dwind-route-in 420ms cubic-bezier(0.16, 1, 0.3, 1) both;
}

/* ------------------------------------------------------------------ */
/* docs prose                                                           */
/* ------------------------------------------------------------------ */

.dw-scrollbar::-webkit-scrollbar { width: 10px; height: 10px; }
.dw-scrollbar::-webkit-scrollbar-track { background: transparent; }
.dw-scrollbar::-webkit-scrollbar-thumb {
    background: #26262C;
    border-radius: 8px;
    border: 3px solid transparent;
    background-clip: content-box;
}
.dw-scrollbar::-webkit-scrollbar-thumb:hover { background: #3A3A44; background-clip: content-box; }

/* ------------------------------------------------------------------ */
/* motion preferences                                                   */
/* ------------------------------------------------------------------ */

@media (prefers-reduced-motion: reduce) {
    .dw-marquee-track,
    .dw-sheen,
    .dw-word { animation: none !important; }

    .dw-sheen { color: #D5B65F; }
    .dw-spot { transform: none !important; }
}
"#;
