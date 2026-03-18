use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

fn parallax_style(speed: f64, extra: &'static str) -> String {
    if extra.is_empty() {
        format!(
            "transform: translate3d(0, calc(var(--scroll-y) * {speed:.3}), 0); will-change: transform;"
        )
    } else {
        format!(
            "transform: translate3d(0, calc(var(--scroll-y) * {speed:.3}), 0) {extra}; will-change: transform;"
        )
    }
}

fn parallax_style_with_x(x: &'static str, speed: f64, extra: &'static str) -> String {
    if extra.is_empty() {
        format!(
            "transform: translate3d({x}, calc(var(--scroll-y) * {speed:.3}), 0); will-change: transform;"
        )
    } else {
        format!(
            "transform: translate3d({x}, calc(var(--scroll-y) * {speed:.3}), 0) {extra}; will-change: transform;"
        )
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/testing_ui.css"/>
        <Title text="Motion Stack Portfolio"/>
        <Meta
            name="description"
            content="Parallax-heavy portfolio concept built with Leptos."
        />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Routes>
                <Route path="" view=HomePage/>
            </Routes>
        </Router>
    }
}

#[component]
fn Metric(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="metric">
            <span class="metric-label">{label}</span>
            <span class="metric-value">{value}</span>
        </div>
    }
}

#[component]
fn ProjectCard(
    class_name: &'static str,
    index: &'static str,
    title: &'static str,
    body: &'static str,
    detail: &'static str,
) -> impl IntoView {
    let class_name = format!("project-card {class_name}");

    view! {
        <article class=class_name>
            <span class="project-index">{index}</span>
            <p class="project-detail">{detail}</p>
            <h3>{title}</h3>
            <p>{body}</p>
        </article>
    }
}

#[component]
fn VideoCard(
    class_name: &'static str,
    title: &'static str,
    embed_url: &'static str,
    caption: &'static str,
) -> impl IntoView {
    let class_name = format!("video-card {class_name}");

    view! {
        <article class=class_name>
            <div class="video-frame">
                <iframe
                    src=embed_url
                    title=title
                    loading="lazy"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen=true
                ></iframe>
            </div>
            <div class="video-meta">
                <h3>{title}</h3>
                <p>{caption}</p>
            </div>
        </article>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (scroll_y, set_scroll_y) = create_signal(0.0_f64);

    #[cfg(not(feature = "hydrate"))]
    let _ = set_scroll_y;

    #[cfg(feature = "hydrate")]
    {
        use std::{cell::Cell, rc::Rc};

        let window = window();
        set_scroll_y.set(window.scroll_y().unwrap_or_default());

        let pending = Rc::new(Cell::new(false));
        let handle = window_event_listener(ev::scroll, {
            let pending = pending.clone();
            let window = window.clone();
            move |_| {
                if pending.replace(true) {
                    return;
                }

                let pending = pending.clone();
                let window = window.clone();
                request_animation_frame(move || {
                    pending.set(false);
                    set_scroll_y.set(window.scroll_y().unwrap_or_default());
                });
            }
        });

        on_cleanup(move || handle.remove());
    }

    let page_style = move || format!("--scroll-y: {:.2}px;", scroll_y.get());

    view! {
        <main class="portfolio-page" style=page_style>
            <header class="topbar">
                <a class="brand" href="#top">
                    <span class="brand-mark">"N"</span>
                    <span class="brand-copy">
                        <strong>"NICOL / MOTION"</strong>
                        <span>"actual parallax concept"</span>
                    </span>
                </a>
                <nav class="topnav">
                    <a href="#about">"About"</a>
                    <a href="#work">"Work"</a>
                    <a href="#video">"Video Wall"</a>
                    <a href="#contact">"Contact"</a>
                </nav>
            </header>

            <section id="top" class="scene hero-scene">
                <div class="scene-shell hero-shell">
                    <div
                        class="hero-grid"
                        style=parallax_style(0.12, "perspective(1200px) rotateX(72deg)")
                    ></div>
                    <div
                        class="hero-grid hero-grid-secondary"
                        style=parallax_style(0.07, "perspective(1200px) rotateX(72deg) rotateZ(7deg)")
                    ></div>
                    <div class="hero-haze hero-haze-one" style=parallax_style(0.15, "")></div>
                    <div class="hero-haze hero-haze-two" style=parallax_style(0.22, "")></div>
                    <div class="hero-orbit orbit-a" style=parallax_style(0.1, "rotate(8deg)")></div>
                    <div class="hero-orbit orbit-b" style=parallax_style(0.16, "")></div>
                    <div class="hero-watermark" style=parallax_style(0.06, "")>
                        "STACK"
                    </div>

                    <div class="hero-copy-block">
                        <p class="eyebrow">"Creative developer / actual parallax / scroll obsession"</p>
                        <h1 class="hero-title">
                            <span>"I build"</span>
                            <span>"portfolio"</span>
                            <span>"gravity."</span>
                        </h1>
                        <p class="hero-copy">
                            "No pinned-scroll fakeout. The foreground moves with the document and the decorative
                            planes keep drifting continuously underneath it at slower speeds."
                        </p>
                        <div class="hero-tags">
                            <span>"Constant drift"</span>
                            <span>"Depth"</span>
                            <span>"Motion"</span>
                            <span>"Embedded chaos"</span>
                        </div>
                        <a class="scroll-cue" href="#work">"Drop into the stack"</a>
                    </div>

                    <aside class="info-card intro-card">
                        <p class="card-kicker">"CURRENT MODE"</p>
                        <h2>"Freelance visuals for people who hate flat landing pages."</h2>
                        <p>
                            "Sites for artists, founders, and studios that want a homepage to feel staged,
                            cinematic, and a little bit unstable."
                        </p>
                    </aside>

                    <div class="info-card metrics-card">
                        <Metric label="Motion" value="continuous parallax layers"/>
                        <Metric label="Mood" value="neon dusk + warm glass"/>
                        <Metric label="Delivery" value="Leptos single page build"/>
                    </div>
                </div>
            </section>

            <section id="about" class="scene about-scene">
                <div class="scene-shell about-shell">
                    <div class="about-backdrop" style=parallax_style(0.08, "rotate(-90deg)")>
                        "DEPTH"
                    </div>
                    <div class="about-haze about-haze-one" style=parallax_style(0.12, "")></div>
                    <div class="about-haze about-haze-two" style=parallax_style(0.18, "")></div>

                    <div class="about-panel statement-panel">
                        <p class="eyebrow">"Approach"</p>
                        <h2>"Big type in front. Systems underneath. Motion doing the storytelling."</h2>
                        <p>
                            "The page keeps flowing at full speed while the set dressing drifts underneath it,
                            so the depth reads as actual parallax instead of pinned chapters pretending to move."
                        </p>
                    </div>

                    <div class="about-panel notes-panel">
                        <p class="eyebrow">"What gets built"</p>
                        <ul class="bullet-list">
                            <li>"Portfolio sites with attitude"</li>
                            <li>"Campaign pages with embedded media"</li>
                            <li>"Product launches built like editorials"</li>
                            <li>"Visual systems that survive responsive breakpoints"</li>
                        </ul>
                    </div>

                    <div class="about-panel ribbon-panel">
                        <span>"layout direction"</span>
                        <span>"artful UI systems"</span>
                        <span>"motion-led storytelling"</span>
                        <span>"design engineering"</span>
                    </div>
                </div>
            </section>

            <section id="work" class="scene work-scene">
                <div class="scene-shell work-shell">
                    <div class="work-grid" style=parallax_style(0.08, "rotate(-7deg)")></div>
                    <div class="work-glow work-glow-one" style=parallax_style(0.13, "")></div>
                    <div class="work-glow work-glow-two" style=parallax_style(0.18, "")></div>

                    <div class="section-copy">
                        <p class="eyebrow">"Selected chaos"</p>
                        <h2>"Cards sliding over cards over cards."</h2>
                        <p>
                            "The projects ride the normal document flow. The backplanes and glows lag behind them
                            on purpose so the whole thing keeps a real sense of depth as you move down the page."
                        </p>
                    </div>

                    <div class="projects-stage">
                        <ProjectCard
                            class_name="project-alpha"
                            index="01"
                            title="Signal Burst"
                            detail="Identity site / broadcast graphics / live merch drop"
                            body="A launch page where headlines stay aggressive and the supporting layers keep drifting behind the content instead of freezing it in place."
                        />
                        <ProjectCard
                            class_name="project-beta"
                            index="02"
                            title="Afterlight Archive"
                            detail="Photographer portfolio / essays / edition sales"
                            body="Dense image-led storytelling built around negative space, editorial rhythm, and parallax planes that make the photography feel staged rather than tiled."
                        />
                        <ProjectCard
                            class_name="project-gamma"
                            index="03"
                            title="Static Bloom"
                            detail="Creative studio / reel / client deck"
                            body="A homepage that treats credentials like posters pinned into a moving set, with fragments drifting behind the main narrative at lower speeds."
                        />
                    </div>

                    <div class="timeline-banner">
                        <span>"Brand systems"</span>
                        <span>"Motion language"</span>
                        <span>"Responsive staging"</span>
                        <span>"Build and handoff"</span>
                    </div>
                </div>
            </section>

            <section id="video" class="scene video-scene">
                <div class="scene-shell video-shell">
                    <div class="video-rings" style=parallax_style(0.07, "rotate(14deg)")></div>
                    <div class="video-haze video-haze-one" style=parallax_style(0.12, "")></div>
                    <div class="video-haze video-haze-two" style=parallax_style(0.19, "")></div>

                    <div class="section-copy video-copy-block">
                        <p class="eyebrow">"Video wall"</p>
                        <h2>"Random YouTube energy, treated like floating set pieces."</h2>
                        <p>
                            "The embeds stay in the foreground and the ring system behind them keeps sliding more
                            slowly, so the media stack feels like part of the scene instead of a dead grid."
                        </p>
                    </div>

                    <div class="video-deck">
                        <VideoCard
                            class_name="video-one"
                            title="Studio feed"
                            embed_url="https://www.youtube-nocookie.com/embed/M7lc1UVf-VE?rel=0&modestbranding=1"
                            caption="Official YouTube sample clip used here as a stand-in for ambient studio footage."
                        />
                        <VideoCard
                            class_name="video-two"
                            title="Reference reel"
                            embed_url="https://www.youtube-nocookie.com/embed/ysz5S6PUM-U?rel=0&modestbranding=1"
                            caption="Another sample embed dropped into the stack to make the page feel more like a live board than a brochure."
                        />
                        <VideoCard
                            class_name="video-three"
                            title="Texture pass"
                            embed_url="https://www.youtube-nocookie.com/embed/aqz-KE-bpKQ?rel=0&modestbranding=1"
                            caption="A third floating frame to finish the collage and keep the right side visually unstable."
                        />
                    </div>

                    <p class="video-footnote">
                        "If one of these videos blocks embedding, replace the ID and the layout still holds."
                    </p>
                </div>
            </section>

            <section id="contact" class="scene contact-scene">
                <div class="scene-shell contact-shell">
                    <div
                        class="contact-backdrop"
                        style=parallax_style_with_x("-50%", 0.08, "")
                    ></div>
                    <div
                        class="contact-backdrop contact-backdrop-secondary"
                        style=parallax_style_with_x("-50%", 0.14, "")
                    ></div>

                    <div class="contact-card">
                        <p class="eyebrow">"Last layer"</p>
                        <h2>"Need a portfolio that feels like it is already in motion?"</h2>
                        <p>
                            "Bring the work, the references, or just the vaguest possible art direction. I can
                            turn that into a landing page with real hierarchy instead of a stack of safe rectangles."
                        </p>

                        <div class="contact-links">
                            <a href="mailto:hello@nicol-motion.studio">"hello@nicol-motion.studio"</a>
                            <a href="https://www.youtube.com/" target="_blank" rel="noreferrer">
                                "Dump me into YouTube"
                            </a>
                            <a href="#top">"Back to the top layer"</a>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}
