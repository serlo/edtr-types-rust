use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdtrArticleReference {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdtrArticleSource {
    pub href: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdtrArticleRelatedContent {
    pub articles: Vec<EdtrArticleReference>,
    pub courses: Vec<EdtrArticleReference>,
    pub videos: Vec<EdtrArticleReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdtrArticle {
    pub introduction: Box<EdtrPlugin>,
    pub content: Box<EdtrPlugin>,
    pub exercises: Vec<EdtrPlugin>,
    pub exercise_folder: EdtrArticleReference,
    pub related_content: EdtrArticleRelatedContent,
    pub sources: Vec<EdtrArticleSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdtrArticleIntroduction {
    pub explanation: Box<EdtrPlugin>,
    pub multimedia: Box<EdtrPlugin>,
    pub illustrating: bool,
    pub width: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum EdtrText {
    SimpleText {
        text: String,
        #[serde(default)]
        strong: bool,
        #[serde(default)]
        em: bool,
    },
    NestedText(EdtrMarkupText),
    Empty {},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EdtrMarkupText {
    #[serde(rename = "p")]
    Paragraph { children: Vec<EdtrText> },
    #[serde(rename = "a")]
    Hyperlink {
        href: String,
        children: Vec<EdtrText>,
    },
    #[serde(rename = "unordered-list")]
    UnorderedList { children: Vec<EdtrText> },
    #[serde(rename = "list-item")]
    ListItem { children: Vec<EdtrText> },
    #[serde(rename = "list-item-child")]
    ListItemChild { children: Vec<EdtrText> },
    #[serde(rename = "h")]
    Heading {
        level: usize,
        children: Vec<EdtrText>,
    },
    #[serde(rename = "math")]
    Math {
        src: String,
        inline: bool,
        children: Vec<EdtrText>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdtrImage {
    pub src: String,
    pub alt: Option<String>,
    pub caption: Box<EdtrPlugin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdtrMultimedia {
    pub explanation: Box<EdtrPlugin>,
    pub multimedia: Box<EdtrPlugin>,
    pub illustrating: bool,
    pub width: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdtrSpoiler {
    pub title: String,
    pub content: Box<EdtrPlugin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "plugin", content = "state")]
#[serde(rename_all = "camelCase")]
pub enum EdtrPlugin {
    Article(EdtrArticle),
    ArticleIntroduction(EdtrArticleIntroduction),
    Text(Vec<EdtrText>),
    Image(EdtrImage),
    Rows(Vec<EdtrPlugin>),
    // Fixme: This is the old representation?
    Table(String),
    Multimedia(EdtrMultimedia),
    Spoiler(EdtrSpoiler),
    Injection(String),
}
