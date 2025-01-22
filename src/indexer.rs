use crate::lexer::Lexer;
use crate::types::Model;
use crate::types::DF;
use crate::types::TF;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Result;
use std::path::Path;
use walkdir::DirEntry;
use walkdir::WalkDir;
use xml::reader::XmlEvent;
use xml::EventReader;

fn read_xml_file(file_path: &Path) -> Result<String> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let file_size = reader.get_ref().metadata()?.len() as usize;
    let mut content = String::with_capacity(file_size / 2);
    for event in EventReader::new(reader).into_iter().flatten() {
        if let XmlEvent::Characters(text) = event {
            content.push_str(&text);
            content.push(' ');
        }
    }
    Ok(content)
}

fn index_file(file_path: &Path) -> Result<(TF, DF)> {
    println!("Indexing {}...", file_path.display());

    let content = read_xml_file(file_path)?.chars().collect::<Vec<_>>();
    let tf = Lexer::new(&content).fold(TF::new(), |mut tf, term| {
        *tf.entry(term).or_insert(0) += 1;
        tf
    });

    let df = tf.keys().fold(DF::new(), |mut df, term| {
        *df.entry(term.to_string()).or_insert(0) += 1;
        df
    });

    Ok((tf, df))
}

fn write_index(index_path: &Path, model: &Model) -> Result<()> {
    println!("Writing {}...", index_path.display());

    let index_file = File::create(index_path)?;
    let buf_writer = BufWriter::new(index_file);
    serde_json::to_writer(buf_writer, model)?;

    Ok(())
}

pub(crate) fn read_index(index_path: &Path) -> Result<Model> {
    let index_file = File::open(index_path)?;
    let reader = BufReader::new(index_file);
    let tf_index = serde_json::from_reader::<BufReader<File>, Model>(reader)?;
    Ok(tf_index)
}

pub(crate) fn check_index(index_path: &Path) -> Result<()> {
    println!("Reading {}...", index_path.display());

    let model = read_index(index_path)?;

    println!(
        "{} contains {} files",
        index_path.display(),
        model.tf_index.len()
    );

    Ok(())
}

pub(crate) fn index_folder(folder_path: &Path, index_path: &Path) -> Result<()> {
    fn is_xhtml_file(file: &DirEntry) -> bool {
        file.file_type().is_file() && file.path().extension().is_some_and(|ext| ext == "xhtml")
    }

    let model = WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(is_xhtml_file)
        .fold(Model::default(), |mut model, file| {
            let file_path = file.path();
            match index_file(file_path) {
                Ok((tf, df)) => {
                    model.tf_index.insert(file_path.to_path_buf(), tf);
                    for (term, count) in df {
                        *model.df_index.entry(term).or_insert(0) += count;
                    }
                }
                Err(err) => {
                    eprintln!("Failed to index file {}: {}", file_path.display(), err);
                }
            }
            model
        });

    write_index(index_path, &model)?;

    Ok(())
}
