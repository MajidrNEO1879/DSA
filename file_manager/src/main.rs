use std::fs;
use std::io;

fn main() -> io::Result<()> {
    println!("Choose the criteria for the files: ");
    println!("1- date ");
    println!("2- size ");
    println!("3- name ");
    println!("4- type ");
    // Get the current directory
    let current_dir = std::env::current_dir()?;
    println!("Current directory: {}", current_dir.display());

    // Read the entries in the current directory
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Print each entry's name
        if path.is_file() {
            println!("File: {}", path.display());
        } else if path.is_dir() {
            println!("Directory: {}", path.display());
        }
    }

    Ok(())
}
