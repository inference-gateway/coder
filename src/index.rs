use ignore::WalkBuilder;
use std::{fs, io, path::Path};

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
            if entry.path() == Path::new(".") || !entry.path().is_file() {
                continue;
            }

            let path = entry.path();

            if let Ok(file_content) = fs::read_to_string(path) {
                let key = path.to_string_lossy().trim_start_matches("./").to_string();

                content.push_str(&format!("  {}: |\n", key));

                for line in file_content.lines() {
                    content.push_str(&format!("    {}\n", line));
                }
            }
        }
    }

    Ok(content)
}
