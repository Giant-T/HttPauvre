use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::tcp::ReadHalf,
};

pub enum FileType {
    Html,
    Png,
    NotSupported,
}

impl FileType {
    ///
    /// Retourne le type de fichier selon son extension
    ///
    pub fn from_str(file_name: &str) -> FileType {
        match file_name.split(".").last().unwrap() {
            "png" => FileType::Png,
            "html" => FileType::Html,
            _ => FileType::NotSupported,
        }
    }

    ///
    /// Retourne le type de contenu
    ///
    pub fn get_content_type(&self) -> &str {
        match self {
            FileType::Html => "text/html",
            FileType::Png => "image/png",
            FileType::NotSupported => "text/html",
        }
    }
}

pub struct Request {
    pub file_name: String,
    pub file_type: FileType,
}

impl Request {
    ///
    /// Retourne les informations de la requête http
    ///
    pub async fn from_tcp_reader(reader: &mut BufReader<ReadHalf<'_>>) -> Option<Self> {
        let mut buf = String::new();
        let bytes = reader.read_line(&mut buf).await.unwrap();

        if bytes != 0 && buf.contains("GET") {
            let args = buf.split(" ").collect::<Vec<&str>>();
            println!("{}", args[1]);

            let file_name: &str;
            if args[1].len() == 1 {
                file_name = "index.html";
            } else {
                file_name = &args[1][1..];
            }

            let file_type = FileType::from_str(file_name);

            return Some(Request {
                file_name: file_name.to_owned(),
                file_type,
            });
        }

        return None;
    }
}
