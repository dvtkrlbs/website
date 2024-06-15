use leptos::*;

#[cfg(feature = "ssr")]
use rust_embed::{Embed, RustEmbed};

#[cfg(feature = "ssr")]
#[derive(Embed)]
#[folder = "src/md/"]
struct MarkdownFiles;

#[component]
pub fn Markdown(file_name: String) -> impl IntoView {
    let file = MarkdownFiles::get(&file_name);

    view! {}
}
