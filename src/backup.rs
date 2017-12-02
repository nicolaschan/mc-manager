use files;
use compression;

// use compression::CompressionAlgorithm;
// use compression::{Encode, Encoder};

use chrono::prelude::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::result::Result;
use tar::Builder;

pub enum DeleteMethod {
    Sequential,
    TimeBlock,
    Thinning
}

fn delete_sequential(backup_dir: &Path, max_backups: usize) -> Result<Vec<String>, Box<::std::error::Error>> {
    let mut backups: Vec<PathBuf> = fs::read_dir(backup_dir)?.map(|entry| entry.unwrap().path()).collect();
    if backups.len() <= max_backups { return Ok(vec![]); };
    let number_to_delete = backups.len() - max_backups;

    backups.sort(); 
    let mut deleted = Vec::new();
    for _ in 0..number_to_delete {
        let oldest = backups.pop().unwrap();
        fs::remove_file(&oldest)?;
        let name = oldest.file_name().unwrap();
        let name = name.to_str().unwrap().to_string();
        println!("Deleted old backup {}", name);
        deleted.push(name);
    }

    Ok(deleted)
}

fn delete_thinning(backup_dir: &Path, max_backups: usize) -> Result<Vec<String>, Box<::std::error::Error>> {
    Ok(vec![])
}

fn delete(backup_dir: &Path, max_backups: Option<usize>, method: DeleteMethod) -> Result<Vec<String>, Box<::std::error::Error>> {
    if max_backups == None { return Ok(vec![]); }
    let max_backups = max_backups.unwrap();

    match method {
        DeleteMethod::Sequential => delete_sequential(backup_dir, max_backups),
        DeleteMethod::TimeBlock => delete_sequential(backup_dir, max_backups),
        DeleteMethod::Thinning => delete_sequential(backup_dir, max_backups)
    }
}

fn save_backup(server_dir: &Path, backup_dir: &Path, verbose: bool) -> Result<(), Box<::std::error::Error>> {
    // Determine path to world folder
    let world_path = server_dir.join("world");
    let timestamp = Local::now().format("%F_%H-%M-%S").to_string();
    let backup_file_name = format!("{}.tar.gz", timestamp);
    
    // Perform backup
    // Thanks to: https://stackoverflow.com/a/46521163/8706910
    let compressed_path = backup_dir.join(backup_file_name);
    if verbose { println!("Backing up {:?} to {:?}", world_path, compressed_path); }
    let compressed_file = File::create(compressed_path)?;
    // let mut encoder = Encoder::new(compressed_file, CompressionAlgorithm::Gzip);
    let mut encoder = GzEncoder::new(compressed_file, Compression::Default);

    {
        let mut archive = Builder::new(&mut encoder);
        archive.append_dir_all("world", world_path)
            .expect("No world exists!");
    }

    encoder.finish()?;
    Ok(())
}

pub fn backup(server: String, backup_dir: Option<String>, max_backups: Option<usize>, verbose: bool) -> Result<(), Box<::std::error::Error>> {
    let server_dir_path = Path::new(&server);

    // Determine which backup directory to use
    let backup_dir = match backup_dir {
        Some(d) => d,
        None => files::get_config(&server)?.backup.dir
    };
    let backup_dir_path = PathBuf::from(&backup_dir);
    let backup_dir_path = match backup_dir_path.is_relative() {
        true => server_dir_path.join(&backup_dir),
        false => backup_dir_path
    };
    let backup_dir_path = backup_dir_path.as_path();
    fs::create_dir_all(&backup_dir_path)?;

    save_backup(&server_dir_path, &backup_dir_path, verbose)?;
    delete(&backup_dir_path, max_backups, DeleteMethod::Sequential)?;

    Ok(())
}

