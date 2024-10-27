#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(dead_code)]
#[serde(tag = "type")]
pub struct MetaData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: Type,
    pub data: Data,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Data {
    Book(Book),
    Doc(Doc),
    Test(Test),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(dead_code)]
pub struct Book {
    pub authors: Vec<String>,
    pub content: Option<Vec<String>>,
    pub edition: Option<String>,
    pub filetype: String,
    pub isbn: String,
    pub isbn_raw: String,
    pub md5: String,
    pub publisher: String,
    pub title: String,
    pub translators: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(dead_code)]
pub struct Course {
    pub name: Option<String>,
    pub type_: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(dead_code)]
pub struct Doc {
    pub content: Option<Vec<String>>,
    pub course: Course,
    pub filetype: String,
    pub md5: String,
    pub title: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "book")]
    Book { id: String, data: Book },
    #[serde(rename = "doc")]
    Doc { id: String, data: Doc },
    #[serde(rename = "test")]
    Test { id: String, data: Test },
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(dead_code)]
pub struct Test {
    pub college: Option<String>,
    pub content: Vec<String>,
    pub course: Course,
    pub filesize: Option<u64>,
    pub filetype: String,
    pub md5: String,
    pub stage: Option<String>,
    pub time: Option<String>,
    pub title: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Type {
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "doc")]
    Doc,
    #[serde(rename = "test")]
    Test,
}

impl From<Item> for MetaData {
    fn from(item: Item) -> Self {
        match item {
            Item::Book { id, data, .. } => MetaData {
                id,
                type_: Type::Book,
                data: Data::Book(data),
            },
            Item::Doc { id, data, .. } => MetaData {
                id,
                type_: Type::Doc,
                data: Data::Doc(data),
            },
            Item::Test { id, data, .. } => MetaData {
                id,
                type_: Type::Test,
                data: Data::Test(data),
            },
        }
    }
}
