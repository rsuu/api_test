#[derive(Debug, PartialEq, Eq)]
pub struct List {
    pub id: usize,
    pub content: Content,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Content {
    pub t: ContentType,
    pub txt: String,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ContentType {
    Img,
    Text,
}
