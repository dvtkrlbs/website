use leptos::*;
use leptos_meta::{Body, Html};
use leptos_router::{use_location, Outlet, A};
use leptos_use::{use_cookie, use_preferred_dark, utils::FromToStringCodec};

use crate::site_config::{Link, SiteConfig};

#[component]
pub fn BaseLayout() -> impl IntoView {
    view! {
        <Body class="bg-main text-main"/>
        <div class="flex flex-col min-h-screen md:px-9">
            <Nav/>
            <Header/>
            <main class="grow w-full max-w-3xl mx-auto">
                <Outlet/>
            </main>
            <Footer/>
        </div>
    }
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
                <ThemeToggle/>
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
    let (subtitle, _) = create_signal(config.subtitle);

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

            <Show when=move || subtitle.get().is_some()>
                <p class="text-sm leading-tight mt-1">{subtitle.get().unwrap()}</p>
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
            "{social_links_base_classlist} sm:flex-row-reverse sm:justify-between sm:items-center"
        )
    };

    let copyright = format!("&copy; {}&nbsp;", time::OffsetDateTime::now_utc().year());

    view! {
        <footer class="w-full max-w-3xl mx-auto pt-12 pb-10 sm:pt-24 sm:pb-14">
            <Show when=move || !nav_links.get().is_empty()>
                <div class="mb-4 flex flex-wrap gap-x-6 gap-y-1">
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

                </div>

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
                    <span inner_html=copyright></span>
                    <a class="hover:underline hover:underline-offset-2" href="/">
                        {title}
                    </a>
                    . All rights reserved.
                </p>
            </div>

        </footer>
    }
}

#[island]
fn ThemeToggle() -> impl IntoView {
    let is_dark_preferred = use_preferred_dark();
    // let (theme, set_theme, _) =
    // use_local_storage_with_options::<String, FromToStringCodec>("theme", options);

    let (theme, set_theme) = use_cookie::<String, FromToStringCodec>("theme");

    create_effect(move |_| {
        if theme.get_untracked().is_none() {
            if is_dark_preferred.get_untracked() {
                set_theme.set(Some("dark".to_string()));
            } else {
                set_theme.set(Some("light".to_string()));
            }
        }
    });

    let html_classes = move || {
        if theme.get() == Some("dark".to_string()) {
            "dark".to_string()
        } else {
            "".to_string()
        }
    };

    view! {
        <Html lang="en" class=move || html_classes()/>
        <button
            class="w-8 h-8 -mr-2 flex items-center justify-center"
            aria-label="Change color scheme"
            on:click=move |_| {
                if let Some(theme) = theme() {
                    if theme == "dark" {
                        set_theme.set(Some("light".to_string()));
                    } else {
                        set_theme.set(Some("dark".to_string()));
                    }
                }
            }
        >

            <svg
                class="w-4 h-4 fill-current"
                viewBox="0 0 16 16"
                xmlns="http://www.w3.org/2000/svg"
            >
                <circle cx="8" cy="8" r="8"></circle>
            </svg>
        </button>
    }
}
