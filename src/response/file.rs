use std::str::FromStr;

use crate::status::HttpStatusCode;

pub enum FileType {
    Html,
    Png,
    PlainText,
}

impl FileType {
    ///
    /// Retourne le type de contenu
    ///
    pub fn get_content_type(&self) -> &str {
        match self {
            FileType::Html => "text/html",
            FileType::Png => "image/png",
            FileType::PlainText => "text/plain",
        }
    }
}

impl FromStr for FileType {
    type Err = HttpStatusCode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(".").last().unwrap() {
            "html" => Ok(FileType::Html),
            "png" => Ok(FileType::Png),
            _ => Ok(FileType::PlainText)
        }
    }
}
