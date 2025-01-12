use crate::lexer::Lexer;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;
use xml::reader::XmlEvent;
use xml::EventReader;

#[allow(clippy::upper_case_acronyms)]
type TFI = HashMap<PathBuf, TF>;
type TF = HashMap<String, usize>;

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
    fn to_uppercase_string(token: &[char]) -> String {
        token
            .iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>()
    }

    println!("Indexing {p}...", p = &file_path.display());

    let content = read_xml_file(file_path)?.chars().collect::<Vec<_>>();
    let tf = Lexer::new(&content)
        .map(to_uppercase_string)
        .fold(TF::new(), |mut tf, term| {
            if let Some(freq) = tf.get(&term) {
                tf.insert(term, freq + 1);
            } else {
                tf.insert(term, 1);
            }
            tf
        });

    Ok(tf)
}

fn write_index(index_path: &Path, tf_index: &TFI) -> io::Result<()> {
    println!("Writing {p}...", p = index_path.display());

    let index_file = fs::File::create(index_path)?;
    serde_json::to_writer(index_file, tf_index)?;

    Ok(())
}

pub(crate) fn check_index(index_path: &Path) -> io::Result<()> {
    println!("Reading {p}...", p = index_path.display());

    let index_file = fs::File::open(index_path)?;
    let tf_index = serde_json::from_reader::<fs::File, TFI>(index_file)?;

    println!(
        "{p:?} contains {count} files",
        p = &index_path,
        count = tf_index.len()
    );

    Ok(())
}

pub(crate) fn index_folder(folder_path: &Path, index_path: &Path) -> io::Result<()> {
    fn is_xhtml_file(file: &DirEntry) -> bool {
        file.file_type().is_file()
            && file
                .file_name()
                .to_str()
                .is_some_and(|name| name.ends_with("xhtml"))
    }

    let tf_index = WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(is_xhtml_file)
        .fold(TFI::new(), |mut tfi, file| {
            let file_path = file.path();
            match index_file(file_path) {
                Ok(tf) => {
                    tfi.insert(file_path.to_path_buf(), tf);
                }
                Err(err) => {
                    eprintln!("Failed to index file {}: {}", file_path.display(), err);
                }
            }
            tfi
        });

    write_index(index_path, &tf_index)?;

    Ok(())
}
