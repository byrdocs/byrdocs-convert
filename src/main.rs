use std::path::Path;

use structopt::StructOpt;
use serde_yaml;
use serde_json;

#[derive(structopt::StructOpt)]
struct Input {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() {
    let input = Input::from_args().input;
    println!("Parse: {}", input.display());
    let input = std::fs::read_to_string(input).unwrap();

    let raw = serde_json::from_str::<serde_yaml::Value>(&input).unwrap();
    
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        std::fs::create_dir(output_dir).unwrap();
    }

    let sum = raw.as_sequence().unwrap().len();
    for metadata in raw.as_sequence().unwrap() {
        let metadata = metadata.clone();
        let id = metadata.get("id").unwrap().as_str().unwrap();
        let mut metadata = metadata.as_mapping().unwrap().clone();
        if let Some(data) = metadata.get_mut("data") {
            if data.is_mapping() {
            data.as_mapping_mut().unwrap().remove("isbn_raw");
            }
        }
        let filetype = if let Some(filetype) = metadata.get("filetype") {
            filetype.as_str().unwrap_or("pdf").to_string()
        } else {
            "pdf".to_string()
        };
        metadata.insert(
            serde_yaml::Value::String("url".to_string()),
            serde_yaml::Value::String(format!("https://byrdocs.org/files/{}.{}", id, filetype)),
        );
        
        let output = output_dir.join(format!("{}.yml", id));
        println!("Write: {}", output.display());
        std::fs::write(output, serde_yaml::to_string(&metadata).unwrap()).unwrap();
    }
    println!("Total: {}", sum);
}
