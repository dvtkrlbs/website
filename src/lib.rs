pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod markdown;
#[cfg(not(feature = "ssr"))]
pub mod markdown {
    use leptos::*;

    #[component]
    pub fn Markdown(file_name: String) -> impl IntoView {
        view! {
            <div>
                <p>Markdown is not supported in this build.</p>
            </div>
        }
    }
}
pub mod pages;
pub mod site_config;

#[cfg(feature = "ssr")]
pub mod fallback;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    // use crate::app::App;

    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // leptos::mount_to_body(App);
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
