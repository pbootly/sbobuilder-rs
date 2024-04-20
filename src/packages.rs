use crate::info_file_parser::SboInfo;
use std::{collections::HashMap, fs};
use walkdir::WalkDir;

fn get_package(path: String, package_name: String) -> Option<SboInfo> {
    let mut sbo_info = SboInfo::new();
    let package_file = format!("{}.info", package_name);
    for entry in WalkDir::new(path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name().to_string_lossy();
        if filename == package_file {
            let contents = fs::read_to_string(entry.path()).unwrap();
            let _ = sbo_info.from_str(&contents);
        }
    }
    Some(sbo_info)
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    children: Vec<Node>,
    details: SboInfo,
}

impl Node {
    pub fn new(name: String, children: Vec<Node>, details: SboInfo) -> Node {
        Node {
            name,
            children,
            details,
        }
    }
}

pub fn build_package_tree(
    path: &str,
    package_name: &str,
    _packages: &HashMap<String, SboInfo>,
) -> Node {
    let mut info = SboInfo::new();
    let mut tree = Node::new(package_name.to_string(), Vec::new(), info.clone());
    let mut children = Vec::new();
    if let Some(package) = get_package(path.to_string(), package_name.to_string()) {
        info = package.clone();
        for dependency in &package.requires {
            let child_tree = build_package_tree(path, dependency, _packages);
            children.push(child_tree);
        }
    }
    tree.children = children;
    tree.name = info.program_name.clone();
    tree.details = info.clone();
    tree
}
