extern crate clap;
extern crate flate2;
extern crate reqwest;
extern crate tar;
extern crate toml;

use files;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::result::Result;
use tar::Builder;

pub fn backup(server: String, backup_dir: Option<String>, verbose: bool) -> Result<(), Box<::std::error::Error>> {
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
    fs::create_dir_all(&backup_dir_path)?;

    // Determine path to world folder
    let world_path = server_dir_path.join("world");
    
    // Perform backup
    // Thanks to: https://stackoverflow.com/a/46521163/8706910
    let compressed_path = backup_dir_path.join("backup.tar.gz");
    if verbose { println!("Backing up {:?} to {:?}", world_path, compressed_path); }
    let compressed_file = File::create(compressed_path)?;
    let mut encoder = GzEncoder::new(compressed_file, Compression::Default);

    {
        let mut archive = Builder::new(&mut encoder);
        archive.append_dir_all("world", world_path)?;
    }

    encoder.finish()?;
    
    Ok(())
}


