use gray_matter::{Matter, ParsedEntity, Result};
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Read},
    path::Path,
};

#[derive(Deserialize, Debug)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub id: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub fn parse_frontmatter(path: &Path) -> std::option::Option<Frontmatter> {
    use gray_matter::engine::YAML;
    println!("{:?}", path);
    let mut contents = String::new();
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                eprintln!("Couldn't open {:?}, not found", path);
                return None;
            }
            ErrorKind::PermissionDenied => {
                eprintln!("Couldn't open {:?}, permission denied", path);
                return None;
            }
            _ => {
                eprintln!("An error occurred accessing {:?}", path);
                return None;
            }
        },
    };

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(e) => eprintln!("An error occurred reading {:?}: {}", file, e),
    }

    let matter = Matter::<YAML>::new();

    let result_with_struct = match matter.parse::<Frontmatter>(&contents) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("An error occurred parsing frontmatter: {}", e);
            return None;
        }
    };

    return result_with_struct.data;
}

pub fn get_all_links(path: &Path)
