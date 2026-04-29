use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    const STYLE: &str = r#"
        :root {
            color-scheme: dark;
            --bg: #05060a;
            --bg-soft: #0b1020;
            --panel: rgba(10, 16, 32, 0.78);
            --panel-strong: rgba(15, 22, 42, 0.94);
            --line: rgba(124, 255, 208, 0.18);
            --text: #e7fff4;
            --muted: rgba(231, 255, 244, 0.72);
            --accent: #77ffd1;
            --accent-2: #78a8ff;
            --warning: #ff8a65;
            --shadow: 0 24px 80px rgba(0, 0, 0, 0.45);
        }

        * {
            box-sizing: border-box;
        }

        html {
            scroll-behavior: smooth;
        }

        body {
            margin: 0;
            min-height: 100vh;
            background:
                radial-gradient(circle at top left, rgba(119, 255, 209, 0.18), transparent 32%),
                radial-gradient(circle at top right, rgba(120, 168, 255, 0.16), transparent 28%),
                linear-gradient(180deg, #070b14 0%, #04050a 60%, #020308 100%);
            color: var(--text);
            font-family: "Space Grotesk", "Segoe UI", "Trebuchet MS", sans-serif;
        }

        body::before {
            content: "";
            position: fixed;
            inset: 0;
            pointer-events: none;
            background-image: linear-gradient(rgba(255, 255, 255, 0.025) 1px, transparent 1px);
            background-size: 100% 4px;
            mix-blend-mode: screen;
            opacity: 0.14;
        }

        a {
            color: inherit;
            text-decoration: none;
        }

        .page {
            position: relative;
            overflow: hidden;
        }

        .wrap {
            width: min(1180px, calc(100% - 32px));
            margin: 0 auto;
        }

        .nav {
            position: sticky;
            top: 0;
            z-index: 10;
            backdrop-filter: blur(18px);
            background: linear-gradient(180deg, rgba(4, 7, 14, 0.82), rgba(4, 7, 14, 0.5));
            border-bottom: 1px solid rgba(119, 255, 209, 0.12);
        }

        .nav-inner {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 20px;
            padding: 18px 0;
        }

        .brand {
            display: flex;
            align-items: center;
            gap: 12px;
            font-family: "JetBrains Mono", "SFMono-Regular", monospace;
            letter-spacing: 0.18em;
            text-transform: uppercase;
            font-size: 0.84rem;
        }

        .brand-mark {
            width: 38px;
            height: 38px;
            border-radius: 12px;
            border: 1px solid rgba(119, 255, 209, 0.3);
            background: linear-gradient(145deg, rgba(119, 255, 209, 0.2), rgba(120, 168, 255, 0.08));
            display: grid;
            place-items: center;
            box-shadow: inset 0 0 18px rgba(119, 255, 209, 0.12);
        }

        .brand-mark::after {
            content: ">_";
            color: var(--accent);
            font-size: 0.95rem;
        }

        .nav-links {
            display: flex;
            gap: 18px;
            flex-wrap: wrap;
            justify-content: flex-end;
            color: var(--muted);
            font-family: "JetBrains Mono", "SFMono-Regular", monospace;
            font-size: 0.86rem;
        }

        .nav-links a {
            transition: color 180ms ease, transform 180ms ease;
        }

        .nav-links a:hover {
            color: var(--accent);
            transform: translateY(-1px);
        }

        .hero {
            position: relative;
            padding: 92px 0 56px;
        }

        .hero-grid {
            display: grid;
            grid-template-columns: minmax(0, 1.25fr) minmax(320px, 0.75fr);
            gap: 28px;
            align-items: center;
        }

        .eyebrow {
            display: inline-flex;
            align-items: center;
            gap: 10px;
            padding: 10px 14px;
            border: 1px solid rgba(119, 255, 209, 0.16);
            border-radius: 999px;
            background: rgba(8, 13, 24, 0.64);
            color: var(--muted);
            font-family: "JetBrains Mono", "SFMono-Regular", monospace;
            font-size: 0.78rem;
            text-transform: uppercase;
            letter-spacing: 0.16em;
        }

        .eyebrow::before {
            content: "";
            width: 8px;
            height: 8px;
            border-radius: 999px;
            background: var(--accent);
            box-shadow: 0 0 18px var(--accent);
        }

        h1 {
            margin: 18px 0 16px;
            font-size: clamp(3rem, 8vw, 6.5rem);
            line-height: 0.9;
            letter-spacing: -0.06em;
            text-transform: uppercase;
            max-width: 10ch;
        }

        .glitch {
            position: relative;
            display: inline-block;
            text-shadow: 0 0 24px rgba(119, 255, 209, 0.18);
        }

        .glitch::before,
        .glitch::after {
            content: attr(data-text);
            position: absolute;
            inset: 0;
            opacity: 0.65;
        }

        .glitch::before {
            transform: translate(2px, -1px);
            color: var(--accent-2);
            clip-path: inset(0 0 62% 0);
        }

        .glitch::after {
            transform: translate(-2px, 1px);
            color: var(--warning);
            clip-path: inset(46% 0 0 0);
        }

        .lede {
            max-width: 66ch;
            font-size: 1.08rem;
            line-height: 1.7;
            color: var(--muted);
            margin: 0 0 28px;
        }

        .hero-actions {
            display: flex;
            flex-wrap: wrap;
            gap: 14px;
            margin-bottom: 28px;
        }

        .button {
            display: inline-flex;
            align-items: center;
            gap: 10px;
            padding: 13px 18px;
            border-radius: 14px;
            border: 1px solid rgba(119, 255, 209, 0.18);
            background: rgba(8, 13, 24, 0.72);
            color: var(--text);
            font-family: "JetBrains Mono", "SFMono-Regular", monospace;
            font-size: 0.86rem;
            text-transform: uppercase;
            letter-spacing: 0.08em;
            transition: transform 180ms ease, border-color 180ms ease, background 180ms ease;
            box-shadow: 0 8px 30px rgba(0, 0, 0, 0.22);
        }

        .button:hover {
            transform: translateY(-2px);
            border-color: rgba(119, 255, 209, 0.42);
            background: rgba(12, 18, 31, 0.94);
        }

        .button.primary {
            background: linear-gradient(135deg, rgba(119, 255, 209, 0.18), rgba(120, 168, 255, 0.18));
        }

        .status-panel,
        .card,
        .project,
        .quote {
            border: 1px solid rgba(119, 255, 209, 0.14);
            background: linear-gradient(180deg, rgba(12, 18, 31, 0.88), rgba(7, 11, 20, 0.9));
            box-shadow: var(--shadow);
            backdrop-filter: blur(18px);
        }

        .status-panel {
            border-radius: 24px;
            padding: 24px;
            display: grid;
            gap: 18px;
            position: relative;
            overflow: hidden;
        }

        .status-panel::before {
            content: "";
            position: absolute;
            inset: 0;
            background: radial-gradient(circle at top right, rgba(119, 255, 209, 0.14), transparent 34%);
            pointer-events: none;
        }

        .terminal-bar {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 16px;
            color: var(--muted);
            font-family: "JetBrains Mono", "SFMono-Regular", monospace;
            font-size: 0.8rem;
            text-transform: uppercase;
            letter-spacing: 0.1em;
        }

        .lights {
            display: flex;
            gap: 8px;
        }

        .light {
            width: 10px;
            height: 10px;
            border-radius: 999px;
        }

        .light.red { background: #ff6b6b; }
        .light.yellow { background: #ffd166; }
        .light.green { background: #77ffd1; }

        .status-list {
            display: grid;
            gap: 14px;
            position: relative;
        }

        .status-item {
            display: flex;
            justify-content: space-between;
            gap: 16px;
            padding: 14px 0;
            border-top: 1px solid rgba(119, 255, 209, 0.1);
        }

        .status-item:first-child {
            border-top: 0;
            padding-top: 0;
        }

        .status-label {
            display: block;
            color: var(--muted);
            font-size: 0.82rem;
            text-transform: uppercase;
            letter-spacing: 0.14em;
            margin-bottom: 6px;
        }

        .status-value {
            font-size: 1.1rem;
            letter-spacing: -0.02em;
        }

        .metrics {
            display: grid;
            grid-template-columns: repeat(3, minmax(0, 1fr));
            gap: 14px;
        }

        .metric {
            border-radius: 18px;
            padding: 16px;
            border: 1px solid rgba(119, 255, 209, 0.12);
            background: rgba(255, 255, 255, 0.02);
        }

        .metric strong {
            display: block;
            font-size: 1.45rem;
            margin-bottom: 6px;
        }

        section {
            padding: 28px 0;
        }

        .section-head {
            display: flex;
            justify-content: space-between;
            align-items: end;
            gap: 18px;
            margin-bottom: 20px;
        }

        .section-title {
            margin: 0;
            font-size: clamp(1.7rem, 3vw, 2.6rem);
            letter-spacing: -0.04em;
            text-transform: uppercase;
        }

        .section-copy {
            max-width: 54ch;
            margin: 0;
            color: var(--muted);
            line-height: 1.7;
        }

        .card-grid {
            display: grid;
            grid-template-columns: repeat(3, minmax(0, 1fr));
            gap: 16px;
        }

        .card {
            border-radius: 22px;
            padding: 22px;
            min-height: 220px;
            position: relative;
            overflow: hidden;
        }

        .card::after {
            content: "";
            position: absolute;
            inset: auto -10% -20% auto;
            width: 160px;
            height: 160px;
            border-radius: 50%;
            background: radial-gradient(circle, rgba(119, 255, 209, 0.16), transparent 68%);
            pointer-events: none;
        }

        .card h3,
        .project h3 {
            margin: 0 0 12px;
            font-size: 1.25rem;
            letter-spacing: -0.03em;
        }

        .tag-list {
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
            margin-top: 18px;
        }

        .tag {
            padding: 8px 12px;
            border-radius: 999px;
            border: 1px solid rgba(119, 255, 209, 0.14);
            background: rgba(255, 255, 255, 0.03);
            color: var(--muted);
            font-family: "JetBrains Mono", "SFMono-Regular", monospace;
            font-size: 0.78rem;
            letter-spacing: 0.05em;
        }

        .project-list {
            display: grid;
            gap: 16px;
        }

        .project {
            border-radius: 24px;
            padding: 22px;
            display: grid;
            grid-template-columns: minmax(0, 1.1fr) minmax(240px, 0.9fr);
            gap: 18px;
            align-items: center;
        }

        .project-meta {
            color: var(--muted);
            font-family: "JetBrains Mono", "SFMono-Regular", monospace;
            font-size: 0.82rem;
            text-transform: uppercase;
            letter-spacing: 0.12em;
            margin-bottom: 10px;
        }

        .project p,
        .card p,
        .quote p,
        .footer p {
            margin: 0;
            color: var(--muted);
            line-height: 1.7;
        }

        .project-links {
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
            justify-content: flex-end;
        }

        .quote {
            border-radius: 24px;
            padding: 28px;
            display: grid;
            gap: 18px;
        }

        .quote strong {
            display: block;
            font-size: 1rem;
            color: var(--accent);
            font-family: "JetBrains Mono", "SFMono-Regular", monospace;
            text-transform: uppercase;
            letter-spacing: 0.12em;
        }

        .contact-grid {
            display: grid;
            grid-template-columns: minmax(0, 1fr) auto;
            gap: 16px;
            align-items: center;
            padding: 24px;
            border-radius: 24px;
            border: 1px solid rgba(119, 255, 209, 0.14);
            background: linear-gradient(135deg, rgba(119, 255, 209, 0.12), rgba(120, 168, 255, 0.08));
        }

        .footer {
            padding: 24px 0 42px;
            color: var(--muted);
            font-family: "JetBrains Mono", "SFMono-Regular", monospace;
            font-size: 0.84rem;
        }

        @media (max-width: 900px) {
            .hero-grid,
            .project,
            .contact-grid,
            .section-head,
            .nav-inner {
                grid-template-columns: 1fr;
                display: grid;
            }

            .project-links {
                justify-content: flex-start;
            }

            .card-grid,
            .metrics {
                grid-template-columns: 1fr;
            }

            .hero {
                padding-top: 56px;
            }

            .nav-links {
                justify-content: flex-start;
            }
        }

        @media (max-width: 640px) {
            .wrap {
                width: min(100% - 20px, 1180px);
            }

            h1 {
                max-width: 100%;
            }

            .hero {
                padding-bottom: 28px;
            }
        }
    "#;

    view! {
        <style>{STYLE}</style>

        <div class="page">
            <header class="nav">
                <div class="wrap nav-inner">
                    <a class="brand" href="#top">
                        <span class="brand-mark"></span>
                        <span>PORTFOLIO / ZERO DAY</span>
                    </a>
                    <nav class="nav-links" aria-label="Primary">
                        <a href="#mission">Mission</a>
                        <a href="#stack">Stack</a>
                        <a href="#work">Work</a>
                        <a href="#contact">Contact</a>
                    </nav>
                </div>
            </header>

            <main id="top">
                <section class="hero">
                    <div class="wrap hero-grid">
                        <div>
                            <div class="eyebrow">Hackery theme / operator online</div>
                            <h1>
                                <span class="glitch" data-text="Crafting">Crafting</span>
                                <br />
                                stealthy digital systems.
                            </h1>
                            <p class="lede">
                                I build sharp, experimental web experiences with a hacker aesthetic,
                                clean engineering, and enough visual noise to feel like a live terminal.
                                This portfolio is designed to read like a control surface rather than a brochure.
                            </p>
                            <div class="hero-actions">
                                <a class="button primary" href="#work">View selected ops</a>
                                <a class="button" href="#contact">Open comms channel</a>
                            </div>
                            <div class="metrics" aria-label="Key stats">
                                <div class="metric">
                                    <strong>08+</strong>
                                    <span class="status-label">Systems shipped</span>
                                    <span class="section-copy">Frontends, portfolios, and product surfaces with a tactical edge.</span>
                                </div>
                                <div class="metric">
                                    <strong>99%</strong>
                                    <span class="status-label">Signal to noise</span>
                                    <span class="section-copy">Intentional visuals, deliberate interactions, and readable structure.</span>
                                </div>
                                <div class="metric">
                                    <strong>24/7</strong>
                                    <span class="status-label">Available mode</span>
                                    <span class="section-copy">Freelance, collaboration, or one-off builds when the mission is clear.</span>
                                </div>
                            </div>
                        </div>

                        <aside class="status-panel" aria-label="System status">
                            <div class="terminal-bar">
                                <div class="lights" aria-hidden="true">
                                    <span class="light red"></span>
                                    <span class="light yellow"></span>
                                    <span class="light green"></span>
                                </div>
                                <span>live / encrypted / stable</span>
                            </div>
                            <div class="status-list">
                                <div class="status-item">
                                    <div>
                                        <span class="status-label">Current focus</span>
                                        <span class="status-value">Portfolio systems and experimental UI</span>
                                    </div>
                                    <div>
                                        <span class="status-label">Primary stack</span>
                                        <span class="status-value">Rust, Leptos, CSS</span>
                                    </div>
                                </div>
                                <div class="status-item">
                                    <div>
                                        <span class="status-label">Design mode</span>
                                        <span class="status-value">Dark, neon, terminal-inspired</span>
                                    </div>
                                    <div>
                                        <span class="status-label">Delivery style</span>
                                        <span class="status-value">Fast, clean, resilient</span>
                                    </div>
                                </div>
                                <div class="status-item">
                                    <div>
                                        <span class="status-label">Signal</span>
                                        <span class="status-value">Independent builder, detail oriented</span>
                                    </div>
                                </div>
                            </div>
                        </aside>
                    </div>
                </section>

                <section id="mission">
                    <div class="wrap">
                        <div class="section-head">
                            <h2 class="section-title">Mission profile</h2>
                            <p class="section-copy">
                                I turn rough concepts into polished, high-contrast interfaces that feel like tools,
                                not templates.
                            </p>
                        </div>
                        <div class="card-grid">
                            <article class="card">
                                <h3>Interface engineering</h3>
                                <p>
                                    Building layouts that stay disciplined under pressure: responsive, accessible,
                                    and intentionally composed.
                                </p>
                                <div class="tag-list">
                                    <span class="tag">Layout systems</span>
                                    <span class="tag">Accessibility</span>
                                    <span class="tag">Performance</span>
                                </div>
                            </article>
                            <article class="card">
                                <h3>Visual identity</h3>
                                <p>
                                    Creating a memorable atmosphere with scanlines, glow, grids, and motion that
                                    supports the story rather than distracting from it.
                                </p>
                                <div class="tag-list">
                                    <span class="tag">Art direction</span>
                                    <span class="tag">Motion</span>
                                    <span class="tag">CSS craft</span>
                                </div>
                            </article>
                            <article class="card">
                                <h3>Delivery velocity</h3>
                                <p>
                                    Shipping quickly without cutting the corners that matter, so the final result
                                    still feels premium when the project is done.
                                </p>
                                <div class="tag-list">
                                    <span class="tag">Rust</span>
                                    <span class="tag">Leptos</span>
                                    <span class="tag">Iteration</span>
                                </div>
                            </article>
                        </div>
                    </div>
                </section>

                <section id="stack">
                    <div class="wrap quote">
                        <strong>Stack / toolkit</strong>
                        <p>
                            Frontend work in Rust with Leptos, strong CSS architecture, careful information design,
                            and compact interactions that keep the page feeling alive without turning it into noise.
                        </p>
                    </div>
                </section>

                <section id="work">
                    <div class="wrap">
                        <div class="section-head">
                            <h2 class="section-title">Selected operations</h2>
                            <p class="section-copy">
                                A few example project types that fit the same hacker-forward visual language.
                            </p>
                        </div>

                        <div class="project-list">
                            <article class="project">
                                <div>
                                    <div class="project-meta">01 / command center</div>
                                    <h3>Developer dashboard for fast-moving teams</h3>
                                    <p>
                                        A compact, high-density workspace for operational metrics, release notes,
                                        and internal task flow with strong contrast and quick scanning.
                                    </p>
                                </div>
                                <div class="project-links">
                                    <a class="button" href="#contact">Request details</a>
                                </div>
                            </article>

                            <article class="project">
                                <div>
                                    <div class="project-meta">02 / stealth launch</div>
                                    <h3>Product landing page with cinematic pacing</h3>
                                    <p>
                                        A launch page that uses glow, grid overlays, and crisp hierarchy to make a
                                        product feel like something someone discovered, not just clicked.
                                    </p>
                                </div>
                                <div class="project-links">
                                    <a class="button" href="#contact">View approach</a>
                                </div>
                            </article>

                            <article class="project">
                                <div>
                                    <div class="project-meta">03 / signal upgrade</div>
                                    <h3>Personal brand system for founders and engineers</h3>
                                    <p>
                                        A modular visual system with adaptable sections, sharp typography, and a tone
                                        that stays technical without feeling cold.
                                    </p>
                                </div>
                                <div class="project-links">
                                    <a class="button" href="#contact">Start mission</a>
                                </div>
                            </article>
                        </div>
                    </div>
                </section>

                <section id="contact">
                    <div class="wrap">
                        <div class="contact-grid">
                            <div>
                                <h2 class="section-title">Open comms</h2>
                                <p class="section-copy">
                                    If you want a portfolio, landing page, or full product interface with a hacker
                                    theme, I can shape it into something that feels sharp and memorable.
                                </p>
                            </div>
                            <a class="button primary" href="mailto:hello@example.com">hello@example.com</a>
                        </div>
                    </div>
                </section>

                <div class="wrap footer">
                    <p>Built as a single-page Leptos app with a dark terminal-inspired aesthetic.</p>
                </div>
            </main>
        </div>
    }
}