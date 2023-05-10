//! Build script
//!
use anyhow::Result;
use downloader::Downloader;
use flate2::read::GzDecoder;
use std::{env, fs, path::PathBuf};
use walkdir::WalkDir;

fn main() -> Result<()> {
    // release file
    let otlp_version = env::var("OBSV_OTEL_PROTO_VERSION").unwrap_or("0.19.0".to_string());
    let otlp_version = otlp_version.strip_prefix('v').unwrap_or(&otlp_version);
    let release_link = format!("https://github.com/open-telemetry/opentelemetry-proto/archive/refs/tags/v{otlp_version}.tar.gz");
    println!("cargo:warning=downloading: {release_link}");

    // download the zip
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(out_dir);
    let dl_file_name = PathBuf::from(format!("opentelemetry-proto-{otlp_version}.tar.gz"));
    let dl_file = out_dir.join(&dl_file_name);

    let mut downloader = Downloader::builder()
        .download_folder(&out_dir)
        .parallel_requests(1)
        .build()?;
    let dl = downloader::Download::new(&release_link).file_name(&dl_file_name);
    downloader.download(&[dl])?;
    println!("cargo:warning=downloaded at: {}", dl_file.to_string_lossy());

    // decompress the .tar.gz archive
    let tar_gz = fs::File::open(&dl_file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(&out_dir)?;
    // => this creates a folder opentelemetry-proto-0.19.0,
    // and the proto files are insde ./opentelemetry/proto

    // build the protobuf structures
    let target_dir = out_dir.join(format!("opentelemetry-proto-{otlp_version}"));
    let mut protos: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(&target_dir) {
        let entry = entry.unwrap();
        if entry.path().is_file() && entry.path().extension().unwrap_or_default() == "proto" {
            // println!("cargo:warning=PROTO_FILE: {}", entry.path().to_string_lossy());
            protos.push(entry.path().to_owned());
        }
    }
    // /opentelemetry/proto
    prost_build::Config::new()
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile_protos(&protos, &[&target_dir])?;
    println!("cargo:warning=generated rust bindings");

    Ok(())
}
