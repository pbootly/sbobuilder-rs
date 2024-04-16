use sbobuilder_rs::info_file_parser::*;
use std::{collections::HashMap, fs};
use walkdir::WalkDir;

fn main() {
    let path = "./slackbuilds";
    let packages: HashMap<String, SboInfo> = HashMap::new();

    let _ = get_package(path.to_string(), "i3".to_string(), packages);
}

fn get_package(
    path: String,
    package_name: String,
    mut packages: HashMap<String, SboInfo>,
) -> SboInfo {
    let mut sbo_info = SboInfo::new();
    let package_file = format!("{}.info", package_name);
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let info = sbo_info.clone();
        let filename = entry.file_name().to_string_lossy();
        if filename == package_file {
            let contents = fs::read_to_string(entry.path()).unwrap();
            let _ = sbo_info.from_str(&contents);
            packages.insert(info.program_name.clone(), info);
        }
    }
    sbo_info
}
