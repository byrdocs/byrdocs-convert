mod metadata;

use metadata::Data;
use serde_yaml;
use std::{io::Write, path::Path};
use structopt::StructOpt;

#[derive(structopt::StructOpt)]
struct Input {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() {
    let input = Input::from_args().input;
    println!("Parse: {}", input.display());

    for entry in input.read_dir().expect("read_dir call failed") {
        let entry = entry.expect("entry failed");
        let path = entry.path();
        println!("Parse: {}", path.display());
        if path.is_file() {
            let file = std::fs::File::open(&path).expect("file open failed");
            let reader = std::io::BufReader::new(file);
            let data: metadata::MetaData =
                serde_yaml::from_reader(reader).expect("serde_yaml::from_reader failed");
            let mut data_string = String::new();
            match data.data.clone() {
                Data::Test(_) => {
                    data_string.push_str(
                        "# yaml-language-server: $schema=https://byrdocs.org/schema/test.yaml\n\n",
                    );
                }
                Data::Book(_) => {
                    data_string.push_str(
                        "# yaml-language-server: $schema=https://byrdocs.org/schema/book.yaml\n\n",
                    );
                }
                Data::Doc(_) => {
                    data_string.push_str(
                        "# yaml-language-server: $schema=https://byrdocs.org/schema/doc.yaml\n\n",
                    );
                }
            }
            data_string.push_str(
                serde_yaml::to_string(&data)
                    .expect("serde_yaml::to_string failed")
                    .as_str(),
            );
            let output_path = Path::new("output").join(path.file_name().unwrap());
            let output_file = std::fs::File::create(&output_path).expect("file create failed");
            let mut writer = std::io::BufWriter::new(output_file);
            writer.write_all(data_string.as_bytes()).unwrap();
        }
    }
}
