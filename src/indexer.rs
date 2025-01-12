use crate::lexer::Lexer;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use xml::reader::XmlEvent;
use xml::EventReader;

type TF = HashMap<String, usize>;
type TFI = HashMap<PathBuf, TF>;

fn read_xml_file(file_path: &Path) -> io::Result<String> {
    let file = fs::File::open(file_path)?;
    let mut content = String::new();
    for event in EventReader::new(file).into_iter().flatten() {
        if let XmlEvent::Characters(text) = event {
            content.push_str(&text);
            content.push(' ');
        }
    }
    Ok(content)
}

fn index_file(file_path: &Path) -> io::Result<HashMap<String, usize>> {
    println!("Indexing {p}...", p = &file_path.display());

    let content = read_xml_file(file_path)?.chars().collect::<Vec<_>>();

    let mut tf = TF::new();

    for token in Lexer::new(&content) {
        let term = token
            .iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>();

        if let Some(freq) = tf.get(&term) {
            tf.insert(term, freq + 1);
        } else {
            tf.insert(term, 1);
        }
    }

    Ok(tf)
}

fn write_index(index_path: &Path, tf_index: &TFI) -> io::Result<()> {
    println!("Saving {p}...", p = index_path.display());
    let index_file = fs::File::create(index_path)?;
    serde_json::to_writer(index_file, tf_index)?;
    Ok(())
}

pub(crate) fn check_index(index_path: &Path) -> io::Result<()> {
    let index_file = fs::File::open(index_path)?;
    println!("Reading {p}...", p = index_path.display());
    let tf_index: TFI = serde_json::from_reader(index_file)?;
    println!(
        "{p:?} contains {count} files",
        p = &index_path,
        count = tf_index.len()
    );
    Ok(())
}

pub(crate) fn index_folder(folder_path: &Path, index_path: &Path) -> io::Result<()> {
    let mut tf_index = TFI::new();

    for entry in fs::read_dir(folder_path)? {
        match entry {
            Err(err) => eprintln!("Failed to read directory entry: {}", err),
            Ok(file) => match file.file_type() {
                Err(err) => eprintln!("Failed to determine file type: {}", err),
                Ok(file_type) => {
                    if file_type.is_file() {
                        let file_path = file.path();
                        let tf = index_file(&file_path)?;
                        tf_index.insert(file_path, tf);
                    }
                }
            },
        }
    }

    write_index(index_path, &tf_index)?;

    Ok(())
}
