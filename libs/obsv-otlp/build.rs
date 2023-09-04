//! Build script
//!
use anyhow::Result;
use downloader::Downloader;
use flate2::read::GzDecoder;
use std::{env, fs, path::PathBuf};
use walkdir::WalkDir;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    // version
    let otlp_version = env::var("OBSV_OTEL_PROTO_VERSION").unwrap_or("1.0.0".to_string());
    let otlp_version = otlp_version.strip_prefix('v').unwrap_or(&otlp_version);

    // files
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(out_dir);
    let dl_file_name = PathBuf::from(format!("opentelemetry-proto-{otlp_version}.tar.gz"));
    let dl_file = out_dir.join(&dl_file_name);
    // NB: if the specs are already downloaded, we assumed the bindings have been generated
    if dl_file.exists() {
        println!("cargo:warning=OTLP specs already downloaded -> skipped");
        return Ok(());
    }

    // download the specs
    let release_link = format!("https://github.com/open-telemetry/opentelemetry-proto/archive/refs/tags/v{otlp_version}.tar.gz");
    println!("cargo:warning=downloading OTLP specs: {release_link}");
    let mut downloader = Downloader::builder()
        .download_folder(&out_dir)
        .parallel_requests(1)
        .build()?;
    let dl = downloader::Download::new(&release_link).file_name(&dl_file_name);
    downloader.download(&[dl])?;
    println!(
        "cargo:warning=downloaded OTLP specs at: {}",
        dl_file.to_string_lossy()
    );

    // decompress the .tar.gz archive
    let tar_gz = fs::File::open(&dl_file)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(&out_dir)?;
    // => this creates a folder opentelemetry-proto-x.y.z,
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
    tonic_build::configure()
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute(".", r#"#[serde(rename_all = "camelCase")]"#)
        // Span ID is a 16 bytes and the JSON MUST be serialized as a 16 bytes HEX value with no leading 0x
        .field_attribute(
            "opentelemetry.proto.trace.v1.Span.trace_id",
            r#"#[serde(serialize_with = "crate::json::serialize_id", deserialize_with = "crate::json::deserialize_id")]"#,
        )
        // Span ID is a 8 bytes and the JSON MUST be serialized as a 8 bytes HEX value with no leading 0x
        .field_attribute(
            "opentelemetry.proto.trace.v1.Span.span_id",
            r#"#[serde(serialize_with = "crate::json::serialize_id", deserialize_with = "crate::json::deserialize_id")]"#,
        )
        .field_attribute(
            "opentelemetry.proto.trace.v1.Span.parent_span_id",
            r#"#[serde(serialize_with = "crate::json::serialize_id", deserialize_with = "crate::json::deserialize_id")]"#,
        )
        .field_attribute(
            "opentelemetry.proto.collector.trace.v1.ExportTraceServiceResponse.partial_success",
            r#"#[serde(deserialize_with = "crate::json::deserialize_trace_partial_success")]"#,
        )
        .field_attribute(
            "opentelemetry.proto.collector.logs.v1.ExportLogsServiceResponse.partial_success",
            r#"#[serde(deserialize_with = "crate::json::deserialize_logs_partial_success")]"#,
        )
        .field_attribute(
            "opentelemetry.proto.collector.metrics.v1.ExportMetricsServiceResponse.partial_success",
            r#"#[serde(deserialize_with = "crate::json::deserialize_metrics_partial_success")]"#,
        )
        .compile(&protos, &[&target_dir])?;
    println!("cargo:warning=generated rust tonic bindings");

    Ok(())
}
