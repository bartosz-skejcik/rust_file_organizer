use std::{
    collections::HashMap,
    env::args,
    error::Error,
    fs::read_dir,
    path::{Path, PathBuf},
};

use chrono::DateTime;

#[derive(Debug)]
struct Directory {
    _path: String,
    files: Vec<File>,
    directories: Option<Vec<Directory>>,
    should_scan_subdirectories: bool,
}

#[derive(Debug)]
struct File {
    _name: String,
    _filetype: String,
    _created_at: DateTime<chrono::Utc>,
    _modified_at: DateTime<chrono::Utc>,
}

impl Directory {
    fn new(path: &Path) -> Self {
        Self {
            _path: path.to_string_lossy().to_string(),
            files: Vec::new(),
            directories: None,
            should_scan_subdirectories: true,
        }
    }

    fn scan_dir(&self, path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut directory = Self::new(path);

        let entries = read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                if entry_path
                    .to_string_lossy()
                    .split("/")
                    .last()
                    .expect("Failed to get last part of path")
                    .starts_with(".")
                {
                    continue;
                }

                if !self.should_scan_subdirectories {
                    continue;
                }

                let sub_directory = self.scan_dir(&entry_path)?;

                directory.directories = match directory.directories {
                    Some(mut directories) => {
                        directories.push(sub_directory);
                        Some(directories)
                    }
                    None => Some(vec![sub_directory]),
                };
            } else {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();

                let file_type = name_str
                    .split('.')
                    .last()
                    .map(|f| f.to_string())
                    .unwrap_or_else(|| "unknown".to_string());

                let metadata = entry.metadata()?;

                let created_at = metadata.created()?;
                let modified_at = metadata.modified()?;

                let created_at = DateTime::<chrono::Utc>::from(created_at);
                let modified_at = DateTime::<chrono::Utc>::from(modified_at);

                directory.files.push(File {
                    _name: name_str.to_string(),
                    _filetype: file_type,
                    _created_at: created_at,
                    _modified_at: modified_at,
                });
            }
        }

        Ok(directory)
    }

    fn create_directories(&self, root_dir: &Path) -> Result<(), Box<dyn Error>> {
        // Create a map to track unique file types
        let mut file_type_dirs: HashMap<String, bool> = HashMap::new();

        for file in &self.files {
            let type_dir_path = root_dir.join(&file._filetype);

            if !file_type_dirs.contains_key(&file._filetype) {
                if !type_dir_path.exists() {
                    std::fs::create_dir(&type_dir_path)?;
                }
                file_type_dirs.insert(file._filetype.clone(), true);
            }

            let file_path = Path::new(&self._path).join(&file._name);
            let new_file_path = type_dir_path.join(&file._name);

            std::fs::rename(&file_path, &new_file_path)?;
        }

        // Recursively process subdirectories if they exist
        if let Some(directories) = &self.directories {
            for directory in directories {
                directory.create_directories(root_dir)?;
            }
        }

        Ok(())
    }
}

fn prompt(label: &str, default_value: &str) -> String {
    println!("{}", label);
    let mut value = String::new();
    std::io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");

    match value.trim() {
        "" => default_value.to_string(),
        _ => value.trim().to_string(),
    }
}

fn main() {
    let path = args().nth(1).unwrap_or(".".to_string());
    let path = PathBuf::from(path);

    let should_scan_subdirectories = matches!(
        prompt("Should we scan subdirectories? (y/n)", "y")
            .trim()
            .to_lowercase()
            .as_str(),
        "y" | "yes"
    );

    let directory = Directory {
        _path: path.to_string_lossy().to_string(),
        files: Vec::new(),
        directories: None,
        should_scan_subdirectories,
    }
    .scan_dir(&path)
    .expect("Failed to scan directory");

    //println!("{:#?}", directory);

    directory
        .create_directories(&path)
        .expect("Failed to create directories");
}
