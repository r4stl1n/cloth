use eyre::{eyre, Result};
use std::fs;
use std::path::Path;
use crate::agentic::tools::tool::Tool;

pub struct FileManagerTool;

impl Tool for FileManagerTool {
    fn name(&self) -> String {
        "fm".to_string()
    }

    fn example(&self) -> String {
        "fm create example.txt This is the content of my file\n\
         fm delete example.text\n\
         fm read example.txt\n\
         fm list directory".to_string()
    }

    fn description(&self) -> String {
        "Manages fm: create, delete, list, read".to_string()
    }

    fn run(&self, data: &str) -> Result<String> {
        let args: Vec<&str> = data.split_whitespace().collect();

        if args.is_empty() {
            return Err(eyre!("No command provided. Available commands: create, delete, list, read"));
        }

        let command = args[0];

        match command {
            "create" => {
                if args.len() < 3 {
                    return Err(eyre!("Usage: fm create <filename> <content>"));
                }
                let filename = args[1];

                // Everything after the filename is the content
                let content_start_idx = data.find(filename).unwrap() + filename.len() + 1;
                let content = if content_start_idx < data.len() {
                    &data[content_start_idx..]
                } else {
                    "" // Empty content if nothing follows the filename
                };

                fs::write(filename, content)
                    .map_err(|e| eyre!("Failed to create file: {}", e))?;

                Ok(format!("File '{}' created successfully", filename))
            },
            "delete" => {
                if args.len() < 2 {
                    return Err(eyre!("Usage: fm delete <filename>"));
                }
                let filename = args[1];

                if !Path::new(filename).exists() {
                    return Err(eyre!("File '{}' does not exist", filename));
                }

                fs::remove_file(filename)
                    .map_err(|e| eyre!("Failed to delete file: {}", e))?;

                Ok(format!("File '{}' deleted successfully", filename))
            },
            "list" => {
                let dir = if args.len() > 1 { args[1] } else { "." };

                if !Path::new(dir).exists() {
                    return Err(eyre!("Directory '{}' does not exist", dir));
                }

                let mut result = String::new();
                result.push_str(&format!("fm in '{}':\n", dir));

                let entries = fs::read_dir(dir)
                    .map_err(|e| eyre!("Failed to read directory: {}", e))?;

                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let file_type = if path.is_dir() { "dir" } else { "file" };
                        let file_name = path.file_name().unwrap().to_string_lossy();
                        result.push_str(&format!("{} ({})\n", file_name, file_type));
                    }
                }

                Ok(result)
            },
            "read" => {
                if args.len() < 2 {
                    return Err(eyre!("Usage: fm read <filename>"));
                }
                let filename = args[1];

                if !Path::new(filename).exists() {
                    return Err(eyre!("File '{}' does not exist", filename));
                }

                let content = fs::read_to_string(filename)
                    .map_err(|e| eyre!("Failed to read file: {}", e))?;

                Ok(format!("Contents of '{}':\n{}", filename, content))
            },
            _ => Err(eyre!("Unknown command: {}. Available commands: create, delete, list, read", command))
        }
    }
}