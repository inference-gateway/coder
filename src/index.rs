use std::{collections::HashMap, fs, io, path::Path};

use ignore::WalkBuilder;
use serde_yaml::Value;

pub fn build_tree() -> io::Result<String> {
    let mut tree = String::from(".\n");
    let mut previous_depth = 0;

    let walker = WalkBuilder::new(".").hidden(true).git_ignore(true).build();

    for result in walker {
        if let Ok(entry) = result {
            if entry.path() == Path::new(".") {
                continue;
            }

            let depth = entry.depth();
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            // Add the correct prefix for the tree structure
            let prefix = if depth > 0 {
                let mut p = String::new();
                for _ in 1..depth {
                    p.push_str("│   ");
                }
                if depth > previous_depth {
                    p.push_str("├── ");
                } else {
                    p.push_str("└── ");
                }
                p
            } else {
                String::from("")
            };

            tree.push_str(&format!("{}{}\n", prefix, name));
            previous_depth = depth;
        }
    }

    Ok(tree)
}

pub fn build_content() -> io::Result<String> {
    let mut content = String::from("content:\n");
    let walker = WalkBuilder::new(".").hidden(true).git_ignore(true).build();

    for result in walker {
        if let Ok(entry) = result {
            // Skip the root directory and non-files
            if entry.path() == Path::new(".") || !entry.path().is_file() {
                continue;
            }

            let path = entry.path();

            // Read file content
            if let Ok(file_content) = fs::read_to_string(path) {
                let key = path.to_string_lossy().trim_start_matches("./").to_string();

                // Write file entry
                content.push_str(&format!("  {}: |\n", key));

                // Add indented content
                for line in file_content.lines() {
                    content.push_str(&format!("    {}\n", line));
                }
            }
        }
    }

    Ok(content)
}

pub fn extract_file_contents(index_content: &str) -> HashMap<String, String> {
    let mut contents = HashMap::new();

    if let Ok(yaml) = serde_yaml::from_str::<Value>(index_content) {
        if let Some(content) = yaml.get("content") {
            // Process each directory
            if let Some(dirs) = content.as_mapping() {
                for (dir, files) in dirs {
                    let dir_name = dir.as_str().unwrap_or("");

                    // Process files in directory
                    if let Some(files_map) = files.as_mapping() {
                        for (file, content) in files_map {
                            let file_name = file.as_str().unwrap_or("");
                            let file_content = content.as_str().unwrap_or("").to_string();

                            // Create full path (dir/file)
                            let full_path = if dir_name.is_empty() {
                                file_name.to_string()
                            } else {
                                format!("{}/{}", dir_name, file_name)
                            };

                            contents.insert(full_path, file_content);
                        }
                    }
                }
            }
        }
    }

    contents
}
