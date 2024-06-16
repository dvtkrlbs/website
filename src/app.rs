use crate::{
    components::BaseLayout,
    pages::Home,
    site_config::{Link, SiteConfig},
    markdown::Markdown,
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let config = SiteConfig {
        title: "Tuna".to_string(),
        subtitle: Some("Tuna's website".to_string()),
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
        footer_nav_links: vec![
            Link {
                text: "About".to_string(),
                href: "/about".to_string(),
            },
            Link {
                text: "Contact".to_string(),
                href: "/contact".to_string(),
            },
        ],
        social_links: vec![
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
        <Stylesheet id="leptos" href="/pkg/website.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="/" view=move || view! { <BaseLayout/> }>
                    <Route path="" view=move || view! { <Home/> }/>
                    <Route path="contact" view=move || view! { <Markdown file_name="contact".to_string() /> }/>
                </Route>
            </Routes>
        </Router>
    }
}
