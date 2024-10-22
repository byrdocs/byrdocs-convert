use serde::{Serialize, Serializer};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Book {
    pub title: String,
    pub authors: Vec<String>,
    pub translators: Vec<String>,
    pub edition: Option<String>,
    pub publisher: String,
    pub publish_year: Option<String>,
    pub isbn: Vec<String>,
    pub filetype: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Course {
    #[serde(rename = "type")]
    pub type_: Option<CourseType>,
    pub name: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum CourseType {
    本科,
    研究生,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Content {
    原题,
    答案,
}

#[derive(serde::Deserialize)]
pub enum Data {
    Test(Test),
    Book(Book),
    Doc(Doc),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Doc {
    pub title: String,
    pub filetype: String,
    pub course: Course,
    pub content: Vec<DocContent>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum DocContent {
    思维导图,
    题库,
    答案,
    知识点,
    课件,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MetaData {
    pub id: String,
    pub url: String,
    #[serde(rename = "type")]
    pub type_: Type,
    pub data: Data,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Semester {
    First,
    Second,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Stage {
    期中,
    期末,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Test {
    pub title: String,
    pub college: Option<Vec<String>>,
    pub course: Course,
    pub time: Option<Time>,
    pub filetype: String,
    pub content: Vec<Content>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Time {
    pub start: String,
    pub end: String,
    pub semester: Option<Semester>,
    pub stage: Option<Stage>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Type {
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "doc")]
    Doc,
}

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Data::Doc(doc) => doc.serialize(serializer),
            Data::Test(test) => test.serialize(serializer),
            Data::Book(book) => book.serialize(serializer),
        }
    }
}