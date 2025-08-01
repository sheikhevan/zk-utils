use serde::Deserialize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Deserialize, Debug)]
struct Frontmatter {
    title: Option<String>,
    id: Option<String>,
    tags: Option<Vec<String>>,
}

pub fn parse_frontmatter(path: &str) {}
