use core::panic;

use leptos::*;

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use rust_embed::{Embed, RustEmbed};
use serde::Deserialize;
use leptos_meta::Title;

#[derive(Embed)]
#[folder = "src/md/"]
struct MarkdownFiles;


#[derive(Debug, Clone, Deserialize)]
pub struct FrontMatter {
    title: String,
}

#[component]
pub fn Markdown(file_name: String) -> impl IntoView {
    let file = MarkdownFiles::get(&format!("{file_name}.md"));

    let Some(file) = file else {
        panic!("File not found: {}", file_name);
    };

    let options = 
        Options::ENABLE_GFM | Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES | Options::ENABLE_HEADING_ATTRIBUTES  | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS;

    let text = std::str::from_utf8(&file.data).unwrap();

    let mut front_matter: Option<FrontMatter> = None;

    let mut is_front_matter = false;


    let parser = Parser::new_ext(text, options).map(|event| {
        match &event {
            Event::Start(Tag::MetadataBlock(pulldown_cmark::MetadataBlockKind::YamlStyle)) => {
                is_front_matter = true;
            }
            Event::End(TagEnd::MetadataBlock(pulldown_cmark::MetadataBlockKind::YamlStyle)) => {
                is_front_matter = false;
            }
            Event::Text(text) => {
                if is_front_matter {
                    front_matter = Some(serde_yml::from_str::<FrontMatter>(text).unwrap());
                }
            }
            other => {
                dbg!(&other);
            }
        }

        event
    });

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let front_matter = front_matter.unwrap();

    dbg!(&html_output);
    dbg!(&front_matter);

    // let options = Options::
    view! {
        <Title text=front_matter.title />
        <div class="prose font-serif prose-stone dark:prose-invert" inner_html=html_output />
    }
}
