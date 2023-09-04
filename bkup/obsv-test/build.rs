use std::{env, fs, path::PathBuf, process::Command, str::FromStr};

/// Github repository for OpenTelemetry repo
const OTEL_PROTO_REPO: &str = "https://github.com/open-telemetry/opentelemetry-proto";

/// Opentelemetry versions to retrieve
const OTEL_VERSIONS: [&str; 5] = ["v0.15.0", "v0.16.0", "v0.17.0", "v0.18.0", "v0.19.0"];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from_str(out_dir.as_str()).unwrap();
    // println!("cargo:warning=OUT_DIR is {out_dir:?}");

    for v in OTEL_VERSIONS {
        let out_dir = out_dir.join(format!("otel-proto/{v}/"));
        // println!("cargo:warning=Downloading specs {v:?} to {out_dir:?}");

        // download if missing
        if !out_dir.exists() {
            fs::create_dir_all(&out_dir).unwrap();

            // download the OpenTelemetry proto defintions from Github
            match Command::new("git")
                .arg("clone")
                .arg("--single-branch")
                .arg("--branch")
                .arg(v)
                .arg(OTEL_PROTO_REPO)
                .arg(&out_dir)
                .output()
            {
                Ok(otp) => {
                    // println!("cargo:warning=OUT_DIR is {err:?}");
                    if !otp.status.success() {
                        let stderr_str = std::str::from_utf8(&otp.stderr).unwrap();
                        panic!("cannot download proto specs: {stderr_str:?}")
                    }
                    println!("cargo:warning=OK: Specs {v} copied to {out_dir:?}");
                }
                Err(err) => {
                    panic!("cannot download OpenTelemetry repo: {err:?}")
                }
            }
        } else {
            // >> NB: already downloaded
        }
    }

    println!("cargo:warning=OK: OpenTelemetry proto specs copied to OUT_DIR");
}
