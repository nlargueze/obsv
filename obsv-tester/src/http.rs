//! HTTP requests

use std::{env, fs, path::PathBuf, str::FromStr};

use clap::Args;
use colored::Colorize;
use futures::StreamExt;
use hyper::{
    header,
    http::uri::{Authority, PathAndQuery, Scheme},
    Body, Client, Request, Response, Uri,
};

use crate::{
    error::{AdhocError, Error},
    test::{Test, TestCollection},
};

/// HTTP request arguments
#[derive(Debug, Args, Clone)]
pub struct HttpRequestArgs {
    /// HTTP method (eg . GET, POST, PUT, ...)
    method: String,
    /// URL (eg localhost:8080, :8080, ...)
    uri: Uri,
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
        // set URI
        let mut uri_parts = self.uri.clone().into_parts();
        if uri_parts.scheme.is_none() {
            uri_parts.scheme = Some(Scheme::HTTP);
        }
        if uri_parts.path_and_query.is_none() {
            uri_parts.path_and_query = Some(PathAndQuery::from_str("").unwrap());
        }
        if let Some(auth) = &uri_parts.authority {
            if auth.host().is_empty() {
                if auth.as_str().starts_with(':') {
                    let auth_fixed = format!("localhost{}", auth);
                    uri_parts.authority = Some(Authority::from_str(&auth_fixed).unwrap());
                } else {
                    return Err(Error::InvalidRequest(
                        AdhocError("missing host".to_string()).into(),
                    ));
                }
            }
        }
        let uri = Uri::from_parts(uri_parts).unwrap();
        // eprintln!("{uri:?}");

        // set header
        let crate_version = env::var("CARGO_PKG_VERSION").unwrap();
        let header_user_agent = format!("curly/v{crate_version}");

        // set body
        let body = if let Some(p) = &self.file {
            let bytes = match fs::read(p) {
                Ok(ok) => ok,
                Err(err) => {
                    return Err(Error::InvalidRequest(err.into()));
                }
            };
            Body::from(bytes)
        } else {
            Body::empty()
        };

        // request
        let req = match Request::builder()
            .uri(uri)
            .header(header::USER_AGENT, header_user_agent)
            .body(body)
        {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::InvalidRequest(err.into()));
            }
        };

        Ok(req)
    }
}

/// Sends HTTP requests
pub async fn send_requests(args: HttpRequestArgs) -> Result<TestCollection, Error> {
    let client = Client::new();
    let printer = StdoutPrinter::default();

    // Print the request
    let req = args.to_request()?;
    if args.verbose {
        printer.print_request(&req);
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
                        let printer = StdoutPrinter::default();
                        printer.print_response(&res);
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

/// Trait that defines a printer
trait Printer {
    /// Prints a request to stdout
    fn print_request(&self, req: &Request<Body>);

    /// Prints a response to stdout
    fn print_response(&self, res: &Response<Body>);
}

/// Implementation of [Print]
#[derive(Debug, Default)]
struct StdoutPrinter {}

impl Printer for StdoutPrinter {
    fn print_request(&self, req: &Request<Body>) {
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

        // TODO: print body

        println!();
    }

    fn print_response(&self, res: &Response<Body>) {
        // print URI + HTTP version
        let status = res.status();
        let status_code = status.as_u16();
        let status_str = status.canonical_reason().unwrap_or("non standard code");
        match status_code {
            0..=299 => {
                println!("{}: {}", status_code.to_string().green(), status_str.cyan())
            }
            300..=399 => {
                println!(
                    "{}: {}",
                    status_code.to_string().yellow(),
                    status_str.cyan()
                )
            }
            _ => {
                println!("{}: {}", status_code.to_string().red(), status_str.cyan())
            }
        }

        // print headers
        for (h_name, h_value) in res.headers() {
            let h_name = h_name.to_string().cyan();
            let h_value = h_value.to_str().unwrap().white();
            println!("{h_name}: {h_value}");
        }

        // TODO: print body
    }
}
