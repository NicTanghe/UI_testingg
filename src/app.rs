use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
    view! {
        <main class="portfolio-page">
            <header class="topbar">
                <a class="brand" href="#top">
                    <span class="brand-mark">"N"</span>
                    <span class="brand-copy">
                        <strong>"NICOL / MOTION"</strong>
                        <span>"portfolio concept"</span>
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
                <div class="scene-pin hero-pin">
                    <div class="hero-grid"></div>
                    <div class="hero-haze hero-haze-one"></div>
                    <div class="hero-haze hero-haze-two"></div>
                    <div class="hero-orbit orbit-a"></div>
                    <div class="hero-orbit orbit-b"></div>
                    <div class="hero-watermark">"STACK"</div>

                    <div class="hero-copy-block">
                        <p class="eyebrow">"Creative developer / z-index theatre / scroll obsession"</p>
                        <h1 class="hero-title">
                            <span>"I build"</span>
                            <span>"portfolio"</span>
                            <span>"gravity."</span>
                        </h1>
                        <p class="hero-copy">
                            "A loud one-page concept with layered cards, sticky scenes, oversized type,
                            floating gradients, and a few random YouTube embeds dropped into the composition."
                        </p>
                        <div class="hero-tags">
                            <span>"Parallax"</span>
                            <span>"Depth"</span>
                            <span>"Motion"</span>
                            <span>"Embedded chaos"</span>
                        </div>
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
                        <Metric label="Scenes" value="05 layered chapters"/>
                        <Metric label="Mood" value="neon dusk + warm glass"/>
                        <Metric label="Delivery" value="Leptos single page build"/>
                    </div>

                    <a class="scroll-cue" href="#work">"Drop into the stack"</a>
                </div>
            </section>

            <section id="about" class="scene about-scene">
                <div class="scene-pin about-pin">
                    <div class="about-backdrop">"DEPTH"</div>

                    <div class="about-panel statement-panel">
                        <p class="eyebrow">"Approach"</p>
                        <h2>"Big type in front. Systems underneath. Motion doing the storytelling."</h2>
                        <p>
                            "I lean on overlap, sticky layers, sharp contrast, and controlled clutter so a page
                            keeps revealing itself as you move through it."
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
                <div class="scene-pin work-pin">
                    <div class="section-copy">
                        <p class="eyebrow">"Selected chaos"</p>
                        <h2>"Cards sliding over cards over cards."</h2>
                        <p>
                            "These are mock case studies, but the layout language is the point: angled panels,
                            stacked depth, and content that looks like it is fighting for the front layer."
                        </p>
                    </div>

                    <div class="projects-stage">
                        <ProjectCard
                            class_name="project-alpha"
                            index="01"
                            title="Signal Burst"
                            detail="Identity site / broadcast graphics / live merch drop"
                            body="A launch page where headlines lock in place while cards shear past them and product moments surface one layer at a time."
                        />
                        <ProjectCard
                            class_name="project-beta"
                            index="02"
                            title="Afterlight Archive"
                            detail="Photographer portfolio / essays / edition sales"
                            body="Dense image-led storytelling built around negative space, editorial rhythm, and a sense that every block is sliding in from a different plane."
                        />
                        <ProjectCard
                            class_name="project-gamma"
                            index="03"
                            title="Static Bloom"
                            detail="Creative studio / reel / client deck"
                            body="A homepage that treats credentials like posters pinned into a moving set, with fragments drifting behind the primary narrative."
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
                <div class="scene-pin video-pin">
                    <div class="section-copy video-copy-block">
                        <p class="eyebrow">"Video wall"</p>
                        <h2>"Random YouTube energy, treated like floating set pieces."</h2>
                        <p>
                            "The embeds are part moodboard, part proof that media blocks can be dropped into a
                            composition without flattening the page. Swap the IDs later if you want a tighter reel."
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
                <div class="scene-pin contact-pin">
                    <div class="contact-backdrop"></div>

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
