#![allow(non_snake_case)]

use std::{fs, path::PathBuf};

/*
'!' starts a comment
Product Name | Boards on panel | Log file directory | DMC patterns
*/

#[derive(Debug)]
pub struct Product {
    name: String,
    patterns: Vec<String>,
    boards_on_panel: u8,
    log_dir: PathBuf,
}

pub fn load_product_list(src: &str) -> Vec<Product> {
    let mut list = Vec::new();

    if let Ok(fileb) = fs::read_to_string(src) {
        for full_line in fileb.lines() {
            if !full_line.is_empty() && !full_line.starts_with('!') {
                let line = &full_line[0..full_line.find('!').unwrap_or(full_line.len())];

                let parts: Vec<&str> = line.split('|').map(|f| f.trim()).collect();
                if parts.len() < 3 {
                    continue;
                }

                let boards_on_panel = parts[1].parse::<u8>().unwrap_or(1);
                let log_dir = PathBuf::from(parts[2]);

                if log_dir.try_exists().is_ok_and(|x| x) {
                    list.push(Product {
                        name: parts[0].to_owned(),
                        patterns: parts.iter().skip(3).map(|f| f.to_string()).collect(),
                        boards_on_panel,
                        log_dir,
                    });
                }
            }
        }
    } else {
        println!("ERR: source ({src}) not readable!");
    }

    list
}

impl Product {
    pub fn check_serial(&self, serial: &str) -> bool {
        for pattern in &self.patterns {
            if serial[13..].starts_with(pattern) {
                return true;
            }
        }

        false
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_bop(&self) -> u8 {
        self.boards_on_panel
    }

    pub fn get_log_dir(&self) -> &PathBuf {
        &self.log_dir
    }
}
