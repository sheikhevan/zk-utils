use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

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

fn get_md_files(input: String) -> Vec<Result<DirEntry, Error>> {
    let files: Vec<Result<DirEntry, Error>> = WalkDir::new(input)
        .into_iter()
        .filter(|e| {
            return e.as_ref().map_or(false, |f| {
                f.file_name()
                    .to_str()
                    .map(|s| s.ends_with(".md") || s.ends_with(".markdown"))
                    .unwrap_or(false)
            });
        })
        .collect();

    return files;
}

fn parse_md_files(verbose: bool, input: Vec<String>) -> Vec<MarkdownNode> {
    let mut node_vector: Vec<MarkdownNode> = Vec::new();
    let mut title;
    let mut id;
    let mut path;
    let mut tags;
    let mut links;

    for entry in input {
        for md_wrapped in get_md_files(entry) {
            if let Ok(md) = md_wrapped {
                title = "placeholder".to_string();
                id = "001".to_string();
                path = md.path().to_path_buf();
                tags = vec!["tag1".to_string()];
                links = vec![];
                node_vector.push(MarkdownNode {
                    title,
                    id,
                    path,
                    tags,
                    links,
                    group: None,
                });
            } else {
                println!("There was an error getting the markdown files");
            }
        }
    }

    return node_vector;
}

pub fn print_json(verbose: bool, input: Vec<String>) -> std::io::Result<String> {
    let node_vector = parse_md_files(verbose, input);
    println!("{:?}", node_vector);
    Ok("tmp".to_string())
}
