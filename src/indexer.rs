use crate::lexer::Lexer;
use crate::types::TF;
use crate::types::TFI;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::path::Path;
use walkdir::DirEntry;
use walkdir::WalkDir;
use xml::reader::XmlEvent;
use xml::EventReader;

fn read_xml_file(file_path: &Path) -> Result<String> {
    let reader = BufReader::new(File::open(file_path)?);
    let file_size = reader.get_ref().metadata()?.len() as usize;
    let mut content = String::with_capacity(file_size);
    for event in EventReader::new(reader).into_iter().flatten() {
        if let XmlEvent::Characters(text) = event {
            content.push_str(&text);
            content.push(' ');
        }
    }
    Ok(content)
}

fn index_file(file_path: &Path) -> Result<HashMap<String, usize>> {
    println!("Indexing {p}...", p = &file_path.display());

    let content = read_xml_file(file_path)?.chars().collect::<Vec<_>>();
    let tf = Lexer::new(&content) //
        .fold(TF::new(), |mut tf, term| {
            *tf.entry(term).or_insert(0) += 1;
            tf
        });

    Ok(tf)
}

fn write_index(index_path: &Path, tf_index: &TFI) -> Result<()> {
    println!("Writing {p}...", p = index_path.display());

    let index_file = File::create(index_path)?;
    serde_json::to_writer(index_file, tf_index)?;

    Ok(())
}

pub(crate) fn read_index(index_path: &Path) -> Result<TFI> {
    let index_file = File::open(index_path)?;
    let tf_index = serde_json::from_reader::<File, TFI>(index_file)?;
    Ok(tf_index)
}

pub(crate) fn check_index(index_path: &Path) -> Result<()> {
    println!("Reading {p}...", p = index_path.display());

    let tf_index = read_index(index_path)?;

    println!(
        "{p:?} contains {count} files",
        p = &index_path,
        count = tf_index.len()
    );

    Ok(())
}

pub(crate) fn index_folder(folder_path: &Path, index_path: &Path) -> Result<()> {
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
