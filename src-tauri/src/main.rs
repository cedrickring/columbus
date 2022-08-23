#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::fmt::Display;
use std::fs;

struct FileInfo {
    name: String,
    creation_date: String,
    size: u32,
    is_dir: bool,
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileInfo ({}, {}, {}, {})", self.name, self.creation_date, self.size, self.is_dir)
    }
}


struct FileSystem {
    cache: lru::LruCache<String, Vec<FileInfo>>,
}

fn create_file_system() -> FileSystem {
    FileSystem {
        cache: lru::LruCache::new(1024)
    }
}

impl FileSystem {
    fn read_directory(&mut self, path: String) -> Option<&Vec<FileInfo>> {
        if self.cache.contains(&path) {
            return self.cache.get(&path);
        }

        let paths = fs::read_dir(&path).expect("Failed to read file in directory");
        let file_infos = paths.map(|path| FileInfo {
            is_dir: path.as_ref().unwrap().file_type().unwrap().is_dir(),
            size: 0,
            name: path.as_ref().expect("File name").file_name().into_string().unwrap(),
            creation_date: String::from("unknown"),
        }).collect::<Vec<FileInfo>>();

        self.cache.put(path.to_string(), file_infos);
        self.cache.get(&path)
    }
}


fn main() {
    let mut file_system = create_file_system();
    for i in 1..3 {
        let now = std::time::Instant::now();

        let file_infos = file_system.read_directory(String::from(".")).expect("Failed to read directory");
        for info in file_infos {
            println!("{}", info)
        }

        let elapsed = now.elapsed();
        println!("--> Elapsed {}: {:.2?}", i, elapsed);
    }


    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
