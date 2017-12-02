extern crate broadcast;
extern crate pbr;
extern crate reqwest;

use reqwest::header::ContentLength;
use std::fs::File;
use std::path::Path;
use std::io::{copy, Read, Write};

#[derive(Deserialize)]
pub struct Manifest {
    pub latest: Latest,
}
#[derive(Deserialize)]
pub struct Latest {
    pub snapshot: String,
    pub release: String,
}

pub fn version_manifest() -> Result<Manifest, reqwest::Error> {
    return reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")?
        .json();
}

pub fn download_server(path: &Path, version: &String) -> Result<(), Box<::std::error::Error>> {
    let url = format!(
        "https://s3.amazonaws.com/Minecraft.Download/versions/{}/minecraft_server.{}.jar", 
        *version, *version); 

    let mut response = reqwest::get(&url)?;

    let length = response.headers().get::<ContentLength>()
        .expect("No content length provided")
        .to_le();
    let mut pb = pbr::ProgressBar::new(length);
    pb.set_units(pbr::Units::Bytes);
    let mut server_jar_file = File::create(path)?;
    {
        let mut output = broadcast::BroadcastWriter::new(server_jar_file, &mut pb); 
        copy(&mut response, &mut output)?;
    }
    pb.finish_print(&format!("Downloaded minecraft_server.{}.jar", version));
    
    Ok(())
}


