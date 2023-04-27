//! HTTP requests

use std::{env, fs, path::PathBuf};

use clap::Args;
use colored::Colorize;
use futures::StreamExt;
use hyper::{
    header, http::HeaderValue, Body, Client, HeaderMap, Method, Request, Response, Uri, Version,
};

use crate::{
    error::{AdhocError, Error},
    test::{Test, TestSuite},
};

/// HTTP request arguments
#[derive(Debug, Args, Clone)]
pub struct HttpRequestArgs {
    /// HTTP method (eg . GET, POST, PUT, ...)
    method: String,
    /// URL (eg localhost:8080, :8080, ...)
    uri: String,
    /// Displays the complete request
    #[arg(short, long)]
    file: Option<PathBuf>,
    /// Number of tests to run (defaults to 1 if not set)
    #[arg(short, long)]
    x: Option<usize>,
    /// Displays the complete request
    #[arg(short, long)]
    verbose: bool,
}

impl HttpRequestArgs {
    /// Creates the Http request from the CLI arguments
    fn to_request(&self) -> Result<Request<Body>, Error> {
        // method
        let method: Method = match self.method.to_uppercase().parse::<Method>() {
            Ok(m) => m,
            Err(err) => {
                return Err(Error::InvalidRequest(err.into()));
            }
        };

        // URI
        let mut uri_str = self.uri.clone();
        // eprintln!("{uri_str}");
        if uri_str.starts_with(':') {
            // Only the port is provided => add localhost
            uri_str = format!("http://localhost{uri_str}");
        } else if !uri_str.starts_with("http://") && !uri_str.starts_with("https://") {
            // Add http:// to allow parsing
            uri_str = format!("http://{uri_str}");
        }
        let uri = match uri_str.parse::<Uri>() {
            Ok(ok) => ok,
            Err(err) => {
                return Err(Error::InvalidRequest(
                    AdhocError(format!("invalid URI: {err} ({uri_str})")).into(),
                ));
            }
        };
        // eprintln!("{uri:?}");

        // headers
        let crate_version = env::var("CARGO_PKG_VERSION").unwrap();
        let mut headers = HeaderMap::new();
        let h_user_agent = HeaderValue::from_str(&format!("curly/v{crate_version}")).unwrap();
        headers.insert(header::USER_AGENT, h_user_agent);

        // body
        let body = if let Some(p) = &self.file {
            let content_type = if let Some(ext) = p.extension() {
                match ext.to_str().unwrap() {
                    "json" => mime::APPLICATION_JSON,
                    _ => mime::TEXT_PLAIN_UTF_8,
                }
            } else {
                mime::TEXT_PLAIN_UTF_8
            };
            let h_content_type = HeaderValue::from_str(content_type.to_string().as_str()).unwrap();

            let bytes = match fs::read(p) {
                Ok(ok) => ok,
                Err(err) => {
                    return Err(Error::InvalidRequest(err.into()));
                }
            };

            headers.insert(header::CONTENT_TYPE, h_content_type);
            headers.insert(header::CONTENT_LENGTH, bytes.len().into());
            Body::from(bytes)
        } else {
            Body::empty()
        };

        // request
        let mut req_builder = Request::builder()
            .version(Version::HTTP_11)
            .method(method)
            .uri(uri);
        for (k, v) in headers {
            req_builder = req_builder.header(k.unwrap(), v);

            // headers.insert(k.clone(), v.clone());
        }

        let req = match req_builder.body(body) {
            Ok(ok) => ok,
            Err(err) => {
                return Err(Error::InvalidRequest(err.into()));
            }
        };

        Ok(req)
    }
}

/// Sends HTTP requests
pub async fn send_requests(args: HttpRequestArgs) -> Result<TestSuite, Error> {
    let client = Client::new();

    // Print the request
    let req = args.to_request()?;
    if args.verbose {
        print_request(req).await;
    }

    // max number of concurrent streams
    const MAX_CONCURRENT: usize = 1_000;

    let n = if let Some(i) = args.x { i } else { 1 };
    let results = futures::stream::iter(0..n)
        .map(move |i| {
            let args = args.clone();
            let client = client.clone();

            async move {
                // send the request here (WITHOUT awaiting)
                let req = args.to_request().unwrap();
                let test = Test::start(i);
                let res_fut = client.clone().request(req);
                (args, res_fut, test)
            }
        })
        .buffer_unordered(MAX_CONCURRENT)
        .then(|(args, res_fut, mut test)| async move {
            match res_fut.await {
                Ok(res) => {
                    // record the test result
                    let s = res.status();
                    if s.is_informational() || s.is_success() {
                        test.set_ok();
                    } else if s.is_redirection() || s.is_client_error() || s.is_server_error() {
                        test.set_err();
                    }
                    (args, test, Ok(res))
                }
                Err(err) => (args, test, Err(Error::InvalidResponse(err.into()))),
            }
        })
        .then(|(args, test, res)| async move {
            match res {
                Ok(res) => {
                    // print only if one test is running
                    if args.verbose && args.x.is_none() && test.idx == 0 {
                        println!();
                        print_response(res).await;
                    }
                    Ok(test)
                }
                Err(err) => Err(err),
            }
        })
        .collect::<Vec<_>>()
        .await;

    let tests: Result<Vec<Test>, Error> = results.into_iter().collect();
    match tests {
        Ok(vec_tests) => Ok(vec_tests.into()),
        Err(err) => Err(err),
    }
}

/// Prints a [Request<Body>]
async fn print_request(req: Request<Body>) {
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
async fn print_response(res: Response<Body>) {
    // print URI + HTTP version
    let status = res.status();
    let status_code = status.as_u16();
    let status_str = status.canonical_reason().unwrap_or("non standard code");
    match status_code {
        0..=299 => {
            println!(
                "{} {} {}",
                "=>".green(),
                status_code.to_string().green(),
                status_str.green()
            )
        }
        300..=399 => {
            println!(
                "{} {} {}",
                "=>".red(),
                status_code.to_string().yellow(),
                status_str.cyan()
            )
        }
        _ => {
            println!(
                "{} {} {}",
                "=>".red(),
                status_code.to_string().red(),
                status_str.red()
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
        let body_str = std::str::from_utf8(&bytes).unwrap();
        println!();
        println!("{body_str}");
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_add() {
        todo!();
    }
}
