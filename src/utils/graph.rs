use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct GraphData {
    nodes: Vec<MarkdownNode>,
    groups: Option<HashMap<String, Group>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Link {
    raw_link: String,
    target_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Group {
    name: String,
    color: (u8, u8, u8),
}

#[derive(Serialize, Deserialize, Debug)]
struct MarkdownNode {
    title: String,
    id: String,
    path: PathBuf,
    tags: Vec<String>,
    links: Vec<Link>,
    group: Option<String>,
}

pub fn print_json(verbose: bool, input: Vec<String>) -> std::io::Result<String> {
    Ok("tmp".to_string())
}
