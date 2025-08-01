use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_frontmatter(path: &str) {
    let file = File::open(path);

    let reader = BufReader::new(file);

    let mut lines = reader.lines();
}
