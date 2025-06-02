use eyre::{eyre, Result};
use std::fs;
use std::io::Write;
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
         fm write example.txt New content\n\
         fm list".to_string()
    }

    fn description(&self) -> String {
        "Manages fm: create, delete, list, read, write".to_string()
    }

    fn run(&self, data: &str) -> Result<String> {
        // Ensure output directory exists
        let output_dir = Path::new("output");
        if !output_dir.exists() {
            fs::create_dir(output_dir)
                .map_err(|e| eyre!("Failed to create output directory: {}", e))?;
        }

        let args: Vec<&str> = data.split_whitespace().collect();

        if args.is_empty() {
            return Err(eyre!("No command provided. Available commands: create, delete, list, read, write"));
        }

        let command = args[0];

        match command {
            "create" => {
                if args.len() < 3 {
                    return Err(eyre!("Usage: fm create <filename> <content>"));
                }
                let filename = args[1];
                let filepath = output_dir.join(filename);

                // Everything after the filename is the content
                let content_start_idx = data.find(filename).unwrap() + filename.len() + 1;
                let content = if content_start_idx < data.len() {
                    &data[content_start_idx..]
                } else {
                    "" // Empty content if nothing follows the filename
                };

                fs::write(filepath, content)
                    .map_err(|e| eyre!("Failed to create file: {}", e))?;

                Ok(format!("File '{}' created successfully in output directory", filename))
            }
            "delete" => {
                if args.len() < 2 {
                    return Err(eyre!("Usage: fm delete <filename>"));
                }
                let filename = args[1];
                let filepath = output_dir.join(filename);

                if !filepath.exists() {
                    return Err(eyre!("File '{}' does not exist in output directory", filename));
                }

                fs::remove_file(filepath)
                    .map_err(|e| eyre!("Failed to delete file: {}", e))?;

                Ok(format!("File '{}' deleted successfully from output directory", filename))
            }
            "list" => {
                let dir = if args.len() > 1 {
                    output_dir.join(args[1])
                } else {
                    output_dir.to_path_buf()
                };

                if !dir.exists() {
                    return Err(eyre!("Directory '{}' does not exist", dir.display()));
                }

                let mut result = String::new();
                result.push_str(&format!("fm in '{}':\n", dir.display()));

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
            }
            "read" => {
                if args.len() < 2 {
                    return Err(eyre!("Usage: fm read <filename>"));
                }
                let filename = args[1];
                let filepath = output_dir.join(filename);

                if !filepath.exists() {
                    return Err(eyre!("File '{}' does not exist in output directory", filename));
                }

                let content = fs::read_to_string(filepath)
                    .map_err(|e| eyre!("Failed to read file: {}", e))?;

                Ok(format!("Contents of '{}':\n{}", filename, content))
            }
            "write" => {
                if args.len() < 3 {
                    return Err(eyre!("Usage: fm write <filename> <content>"));
                }
                let filename = args[1];
                let filepath = output_dir.join(filename);

                if !filepath.exists() {
                    return Err(eyre!("File '{}' does not exist in output directory", filename));
                }

                let content_start_idx = data.find(filename).unwrap() + filename.len() + 1;
                let content = if content_start_idx < data.len() {
                    &data[content_start_idx..]
                } else {
                    "" // Empty content if nothing follows the filename
                };

                fs::OpenOptions::new()
                    .append(true)
                    .open(filepath)
                    .map_err(|e| eyre!("Failed to open file: {}", e))?
                    .write_all(content.as_bytes())
                    .map_err(|e| eyre!("Failed to write to file: {}", e))?;

                Ok(format!("Successfully wrote to file '{}'", filename))
            }
            _ => Err(eyre!("Unknown command: {}. Available commands: create, delete, list, read, write", command))
        }
    }
}