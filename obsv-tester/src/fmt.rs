//! Formatting

use colored::Colorize;
use hyper::{Body, Request, Response, Uri};

/// Prints a [Request<Body>]
pub async fn print_request(req: Request<Body>) {
    // print URI + HTTP version
    let method = req.method();
    let uri: &Uri = req.uri();
    let uri_host = uri.host().unwrap_or("");
    let uri_path = uri.path();
    let http_vers = format!("{:?}", req.version()).blue();
    println!(
        "{} {} {}",
        method.to_string().green(),
        uri_path.cyan(),
        http_vers.blue()
    );

    // print headers
    for (h_name, h_value) in req.headers() {
        let h_name = h_name.to_string().cyan();
        let h_value = h_value.to_str().unwrap().white();
        println!("{h_name}: {h_value}");
    }
    println!("{}: {}", "host".cyan(), uri_host.white());

    // print body
    let body = req.into_body();
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    if !bytes.is_empty() {
        let body_str = std::str::from_utf8(&bytes).unwrap();
        println!();
        println!("{body_str}");
    }
}

/// Prints a [Response<Body>]
pub async fn print_response(res: Response<Body>) {
    // print URI + HTTP version
    let status = res.status();
    let status_code = status.as_u16();
    let status_str = status.canonical_reason().unwrap_or("non standard code");
    let version_str = format!("{:?}", res.version());
    match status_code {
        0..=299 => {
            println!(
                "{} {} {} {}",
                "=>".green(),
                status_code.to_string().green(),
                status_str.green(),
                version_str.blue(),
            )
        }
        300..=399 => {
            println!(
                "{} {} {} {}",
                "=>".red(),
                status_code.to_string().yellow(),
                status_str.cyan(),
                version_str.blue(),
            )
        }
        _ => {
            println!(
                "{} {} {} {}",
                "=>".red(),
                status_code.to_string().red(),
                status_str.red(),
                version_str.blue(),
            )
        }
    }

    // print headers
    for (h_name, h_value) in res.headers() {
        let h_name = h_name.to_string().cyan();
        let h_value = h_value.to_str().unwrap().white();
        println!("{h_name}: {h_value}");
    }

    // print body
    let body = res.into_body();
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    if !bytes.is_empty() {
        println!();

        match std::str::from_utf8(&bytes) {
            Ok(body_str) => {
                println!("{body_str}");
            }
            Err(_) => {
                println!("{:?}", bytes);
            }
        };
    }
}
