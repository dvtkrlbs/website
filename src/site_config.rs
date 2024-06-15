#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteConfig {
    pub title: String,
    pub subtitle: Option<String>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub text: String,
    pub href: String,
}
