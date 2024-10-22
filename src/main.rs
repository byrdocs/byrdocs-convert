mod new_struct;
mod old_struct;

use std::path::Path;

use new_struct::{DocContent, Semester};
use structopt::StructOpt;
use serde_yaml;
use serde_json;

#[derive(structopt::StructOpt)]
struct Input {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main(){
    let input = Input::from_args().input;
    println!("Parse: {}", input.display());
    let input = std::fs::read_to_string(input).unwrap();
    let output_dir = Path::new("output");

    let raw = serde_json::from_str::<Vec<old_struct::Item>>(&input).unwrap();
    let raw = raw.into_iter().map(|r| old_struct::MetaData::from(r)).collect::<Vec<old_struct::MetaData>>();

    let total = raw.len();
    let mut sum = 0;
    let new = raw.into_iter().map(|old| {
        let mut filetype = String::new();
        let data = match old.data {
            old_struct::Data::Book(book) => new_struct::Data::Book(new_struct::Book{
                title: book.title,
                authors: book.authors,
                translators: book.translators,
                edition: book.edition,
                publisher: book.publisher,
                publish_year: None,
                isbn: vec![book.isbn],
                filetype: {
                    filetype = book.filetype.clone();
                    book.filetype
                },
            }),
            old_struct::Data::Doc(doc) => new_struct::Data::Doc(new_struct::Doc{
                title: doc.title,
                filetype: {
                    filetype = doc.filetype.clone();
                    doc.filetype
                },
                content: doc.content.unwrap().iter().map(
                    |c|
                    match c.as_str() {
                        "思维导图" => new_struct::DocContent::思维导图,
                        "题库" => new_struct::DocContent::题库,
                        "答案" => new_struct::DocContent::答案,
                        "知识点" => new_struct::DocContent::知识点,
                        "课件" => new_struct::DocContent::课件,
                        _ => {
                            panic!("Unknown content type: {} in {}", c, old.id.clone())
                        },
                    }
                ).collect::<Vec<DocContent>>(),
                course: new_struct::Course{
                    type_: match doc.course.type_ {
                        Some(type_) => match type_.as_str() {
                            "本科" => Some(new_struct::CourseType::本科),
                            "研究生" => Some(new_struct::CourseType::研究生),
                            _ => None,
                        },
                        None => None,
                    },
                    name: doc.course.name,
                },
            }),
            old_struct::Data::Test(test) => new_struct::Data::Test(new_struct::Test{
                title: test.title,
                college: match test.college {
                    Some(collage) => Some(collage.split("/").map(|s| s.to_string()).collect::<Vec<String>>()),
                    None => None
                },
                course: new_struct::Course{
                    type_: match test.course.type_ {
                        Some(type_) => match type_.as_str() {
                            "本科" => Some(new_struct::CourseType::本科),
                            "研究生" => Some(new_struct::CourseType::研究生),
                            _ => None,
                        },
                        None => None,
                    },
                    name: test.course.name,
                },
                time: {
                    match test.time {
                        Some(time) => Some({
                            let time_vec = time.split(&['-', ' '][..]).map(|s| s.trim().to_string()).collect::<Vec<String>>();
                            let start_time = time_vec[0].clone();
                            let end_time = match time_vec.get(1) {
                                Some(end_time) => end_time.clone(),
                                None => start_time.clone(),
                            };
                            let semester = match time_vec.get(2) {
                                Some(semester) => match semester.as_str() {
                                    "第一学期" => Some(Semester::First),
                                    "第二学期" => Some(Semester::Second),
                                    _ => None,
                                },
                                None => None,
                            };
                            let stage = match test.stage {
                                Some(stage) => match stage.as_str() {
                                    "期中" => Some(new_struct::Stage::期中),
                                    "期末" => Some(new_struct::Stage::期末),
                                    _ => None,
                                },
                                None => None,
                            };
                            new_struct::Time{
                                start: start_time,
                                end: end_time,
                                semester: semester,
                                stage: stage,
                            }
                        }),
                        None => None
                    }
                    
                },
                filetype: {
                    filetype = test.filetype.clone();
                    test.filetype
                },
                content: test.content.iter().map(
                    |s| 
                    match s.as_str() {
                        "试题" => new_struct::Content::原题,
                        "答案" => new_struct::Content::答案,
                        _ => panic!("Unknown content type: {} in {}", s, old.id.clone()),
                    }
                ).collect::<Vec<new_struct::Content>>(),
            }),
        };
        if filetype.is_empty() {filetype = "zip".to_string()};
        sum += 1;
        new_struct::MetaData{
            id: old.id.clone(),
            url: format!("https://byrdocs.org/files/{}.{}", old.id,filetype.as_str()).to_string(),
            type_: match old.type_ {
                old_struct::Type::Test => new_struct::Type::Test,
                old_struct::Type::Book => new_struct::Type::Book,
                old_struct::Type::Doc => new_struct::Type::Doc,
            },
            data: data
        }
        
    }).collect::<Vec<new_struct::MetaData>>();

    for metadata in new {
        let metadata = metadata;
        
        let output = output_dir.join(format!("{}.yml", &metadata.id));
        println!("Write: {}", output.display());
        std::fs::write(output, serde_yaml::to_string(&metadata).unwrap()).unwrap();
    }
    println!("total: {}\nsuccess: {}", total, sum);
}
