use pulldown_cmark::{Options, Parser};
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Serialize, Deserialize, Debug)]
struct Frontmatter {
    title: Option<String>,
    id: Option<String>,
    tags: Option<Vec<String>>,
}

pub fn parse_frontmatter(path: &str) {}
