use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

use crate::utils::markdown_utils;

#[derive(Serialize, Deserialize, Debug)]
struct GraphData {
    nodes: Vec<MarkdownNode>,
    groups: Option<HashMap<String, Group>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub raw_link: String,
    pub target_id: Option<String>,
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
    let (mut title, mut id, mut path, mut tags, mut links);

    for entry in input {
        if verbose {
            println!("Processing {}", entry);
        }
        for md_wrapped in get_md_files(entry) {
            if let Ok(md) = md_wrapped {
                if let Some(frontmatter) = markdown_utils::parse_frontmatter(md.path()) {
                    title = frontmatter.title.unwrap();
                    id = frontmatter.id.unwrap();
                    path = md.path().to_path_buf();
                    tags = frontmatter.tags.unwrap();
                    links = vec![];
                    node_vector.push(MarkdownNode {
                        title,
                        id,
                        path,
                        tags,
                        links,
                        group: None,
                    });
                }
            } else {
                println!("There was an error accessing the markdown files");
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
