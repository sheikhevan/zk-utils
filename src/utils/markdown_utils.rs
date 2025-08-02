use gray_matter::{Matter, ParsedEntity, Result};
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Read},
    path::Path,
};

#[derive(Deserialize, Debug)]
struct Frontmatter {
    title: Option<String>,
    id: Option<String>,
    tags: Option<Vec<String>>,
}

pub fn parse_frontmatter(path: &Path) {
    println!("{:?}", path);
    let mut contents = String::new();
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                eprintln!("Couldn't open {:?}, not found", path);
                return;
            }
            ErrorKind::PermissionDenied => {
                eprintln!("Couldn't open {:?}, permission denied", path);
                return;
            }
            _ => {
                eprintln!("An error occurred accessing {:?}", path);
                return;
            }
        },
    };

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(e) => eprintln!("An error occurred reading {:?}: {}", file, e),
    }
}
