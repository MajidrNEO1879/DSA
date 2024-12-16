use std::fs; 
use std::env; 

fn list_directory_contents() {
   
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Contents of directory: {}", current_dir.display());
    match fs::read_dir(&current_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        println!("[DIR]  {}", path.display());
                    } else {
                        println!("[FILE] {}", path.display());
                    }
                }
            }
        }
        Err(e) => {
            println!("Error reading directory: {}", e);
        }
    }
}

fn main() {
    list_directory_contents();
}
