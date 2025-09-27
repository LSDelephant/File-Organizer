use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// File Organizer на Rust
#[derive(Parser, Debug)]
#[command(author, version, about = "CLI утиліта для організації файлів")]
struct Args {
    /// Шлях до папки, яку потрібно організувати
    #[arg(default_value = ".")]
    directory: String,
}

fn main() {
    let args = Args::parse();
    let dir = Path::new(&args.directory);

    if !dir.is_dir() {
        eprintln!("❌ Помилка: '{}' не є директорією", dir.display());
        std::process::exit(1);
    }

    println!("📂 Організація файлів у '{}'", dir.display());

    if let Err(e) = organize_files(dir) {
        eprintln!("⚠️ Помилка: {}", e);
    }
}

/// Організує файли у вказаній директорії за розширенням
fn organize_files(dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let folder_name = ext.to_lowercase();
                let new_dir = dir.join(folder_name);
                fs::create_dir_all(&new_dir)?;

                let file_name = path.file_name().unwrap();
                let new_path = new_dir.join(file_name);

                fs::rename(&path, &new_path)?;
                println!("➡️ {} → {}", path.display(), new_path.display());
            } else {
                let new_dir = dir.join("others");
                fs::create_dir_all(&new_dir)?;

                let file_name = path.file_name().unwrap();
                let new_path = new_dir.join(file_name);

                fs::rename(&path, &new_path)?;
                println!("➡️ {} → {}", path.display(), new_path.display());
            }
        }
    }
    Ok(())
}

