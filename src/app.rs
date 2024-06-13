use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let config = SiteConfig {
        title: "Tuna".to_string(),
        logo: None,
        header_nav_links: vec![
            Link {
                text: "Home".to_string(),
                href: "/".to_string(),
            },
            Link {
                text: "Projects".to_string(),
                href: "/projects".to_string(),
            },
        ],
        social_links: vec![
            Link {
                text: "About".to_string(),
                href: "/about".to_string(),
            },
            Link {
                text: "Contact".to_string(),
                href: "/contact".to_string(),
            },
        ],
        footer_nav_links: vec![
            Link {
                text: "Twitter".to_string(),
                href: "https://twitter.com/dvtkrlbs".to_string(),
            },
            Link {
                text: "Mastodon".to_string(),
                href: "https://tunahan.social".to_string(),
            },
        ],
    };

    provide_context(config);

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="/" view=move || view! { <BaseLayout/> }>
                    <Route path="" view=move || view! { <Home/> }/>
                </Route>
            </Routes>
        </Router>
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteConfig {
    pub title: String,
    pub logo: Option<Image>,
    pub header_nav_links: Vec<Link>,
    pub social_links: Vec<Link>,
    pub footer_nav_links: Vec<Link>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub src: String,
    pub alt: Option<String>,
    pub caption: Option<String>,
}

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main></main>
    }
}

#[component]
fn BaseLayout() -> impl IntoView {
    view! {
        <div class="bg-main text-main">
            <div class="flex flex-col min-h-screen md:px-8">
                <Nav/>
                <Header/>
                <main class="grow w-full max-w-3xl max-auto">
                    <Outlet/>
                    <Footer/>
                </main>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    text: String,
    href: String,
}

#[component]
fn Nav() -> impl IntoView {
    let nav_links = use_context::<SiteConfig>().unwrap().header_nav_links;

    let (nav_links, _) = create_signal(nav_links);

    view! {
        <nav class="min-h-10 pt-4 pb-12 relative sm:min-h-14 sm:pb-24 md:pt-8">
            <Show when=move || { !nav_links.get().is_empty() }>
                <div class="w-full max-w-3xl mx-auto relative">
                    <button
                        class="menu-toggle w-8 h-8 -ml-1 flex items-center justify-center relative z-30 md:hidden"
                        aria-label="Open Menu"
                        aria-expanded="false"
                        aria-controls="menu-items"
                    >
                        <span class="menu-toggle-icon w-6 h-px relative bg-current"></span>
                    </button>
                    <ul id="menu-items" class="menu flex gap-6">

                        <For
                            each=move || nav_links.get()
                            key=|nav_link| nav_link.href.clone()
                            children=move |nav_link: Link| {
                                view! {
                                    <li class="py-1">
                                        <NavLink href=nav_link.href>{nav_link.text}</NavLink>
                                    </li>
                                }
                            }
                        />

                    </ul>
                </div>
            </Show>

            <div class="absolute right-0 top-4 z-10 md:top-8">
                <p>Theme Toggle</p>
            </div>
        </nav>
    }
}

#[component]
fn NavLink(href: String, children: Children) -> impl IntoView {
    let location = use_location();
    let href_clone = href.clone();
    let is_active = move || location.pathname.get() == href_clone;

    let base_class_list = "text-xl font-serif text-main hover:underline hover:underline-offset-2 hover:decoration-1 md:text-base";
    let class_list = if is_active() {
        format!("underline underline-offset-2 decoration-1 {base_class_list}")
    } else {
        base_class_list.to_string()
    };

    view! {
        <A href=href class=class_list>
            {children()}
        </A>
    }
}

#[component]
fn Header() -> impl IntoView {
    let config = use_context::<SiteConfig>().unwrap();
    let (logo, _) = create_signal(config.logo);
    let (title, _) = create_signal(config.title);

    view! {
        <header class="w-full max-w-3xl mx-auto mb-12 sm:mb-16">
            <Show
                when=move || logo.get().is_some()
                fallback=move || {
                    view! {
                        <A
                            class="font-serif text-2xl leading-tight font-medium text-theme-foreground sm:text-4xl"
                            href="/"
                        >
                            {title}
                        </A>
                    }
                }
            >

                <A href="/">
                    <img
                        src=logo.get().unwrap().src
                        alt=logo.get().unwrap().alt.unwrap_or_else(|| "".to_string())
                        class="max-h-12"
                    />
                </A>
            </Show>

        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let config = use_context::<SiteConfig>().unwrap();

    let (social_links, _) = create_signal(config.social_links);
    let (nav_links, _) = create_signal(config.footer_nav_links);
    let (title, _) = create_signal(config.title);

    let social_links_base_classlist = "pt-6 flex flex-col gap-4 border-t border-dashed border-main";
    let social_links_classlist = if social_links.get().is_empty() {
        social_links_base_classlist.to_string()
    } else {
        format!(
            "sm:flex-row-reverse sm:justify-between sm:items-center {social_links_base_classlist}"
        )
    };

    view! {
        <footer class="w-full max-w-3xl mx-auto pt-12 pb-10 sm:pt-24 sm:pb-14">
            <Show when=move || !nav_links.get().is_empty()>
                <For
                    each=move || nav_links.get()
                    key=move |nav_link| nav_link.href.clone()

                    children=move |link: Link| {
                        view! {
                            <a
                                class="font-serif hover:underline hover:underline-offset-2"
                                href=link.href
                            >
                                {link.text}
                            </a>
                        }
                    }
                />

            </Show>
            <div class=social_links_classlist>
                <Show when=move || !social_links.get().is_empty()>
                    <div class="flex flex-wrap gap-x-4 gap-y-1">
                        <For
                            each=move || social_links.get()
                            key=move |link| link.href.clone()
                            children=move |link| {
                                view! {
                                    <a
                                        class="inline-flex items-center justify-center text-sm hover:underline hover:underline-offset-2"
                                        href=link.href
                                        target="_blank"
                                        rel="noopener noreferer"
                                    >
                                        {link.text}
                                    </a>
                                }
                            }
                        />

                    </div>
                </Show>
                <p class="text-sm">
                    &copy; {time::OffsetDateTime::now_utc().year()}&nbsp;
                    <a class="hover:underline hover:underline-offset-2" href="/">
                        {title}
                    </a>. All rights reserved.
                </p>
            </div>

        </footer>
    }
}
