use lexer::Lexer;
use std::collections::HashMap;
use std::fs::read_dir;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use xml::reader::EventReader;
use xml::reader::XmlEvent;

mod lexer;

type TF = HashMap<String, usize>;
type TFI = HashMap<PathBuf, TF>;

fn read_xml_file(file_path: &Path) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut content = String::new();
    for event in EventReader::new(file) {
        if let Ok(XmlEvent::Characters(text)) = event {
            content.push_str(&text);
            content.push(' ');
        }
    }
    Ok(content)
}

fn read_index() -> io::Result<()> {
    let index_path = "index.json";
    let index_file = File::open(index_path)?;
    println!("Reading {index_path}...");
    let tf_index: TFI = serde_json::from_reader(index_file)?;
    println!(
        "{index_path} contains {count} files",
        count = tf_index.len()
    );
    Ok(())
}

fn write_index(tf_index: &TFI) -> io::Result<()> {
    let index_path = "index.json";
    println!("Saving {index_path}...");
    let index_file = File::create(index_path)?;
    serde_json::to_writer(index_file, tf_index)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut tf_index = TFI::new();

    for entry in read_dir("docs.gl/gl4")? {
        match entry {
            Err(err) => eprintln!("Failed to read directory entry: {}", err),
            Ok(file) => match file.file_type() {
                Err(err) => eprintln!("Failed to determine file type: {}", err),
                Ok(file_type) => {
                    if file_type.is_file() {
                        let file_path = file.path();

                        println!("Indexing {:?}...", &file_path);

                        let content = read_xml_file(file_path.as_path())?
                            .chars()
                            .collect::<Vec<_>>();

                        let mut tf = TF::new();

                        for token in Lexer::new(&content) {
                            let term = token
                                .into_iter()
                                .map(|c| c.to_ascii_uppercase())
                                .collect::<String>();

                            if let Some(freq) = tf.get(&term) {
                                tf.insert(term, freq + 1);
                            } else {
                                tf.insert(term, 1);
                            }
                        }

                        tf_index.insert(file_path, tf);
                    }
                }
            },
        }
    }

    write_index(&tf_index)?;
    read_index()?;

    Ok(())
}
