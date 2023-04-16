#![deny(warnings)]

use std::sync::Arc;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::env;
use std::net::SocketAddr;
use tokenizers::tokenizer::{Result, Tokenizer};

async fn tokenize(tokenizer: Arc<Tokenizer>, req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, str) => {
            match str {
                "/len" => {
                    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
                    let body_string = String::from_utf8(body_bytes.to_vec())?;
                    let encoded = tokenizer.encode(body_string, false)?;
                    println!("{}",encoded.len());
                    Ok(Response::new(Body::from(encoded.len().to_string())))
                }
                s if s.starts_with("/trim/") => {
                    match s.strip_prefix("/trim/").unwrap().parse::<usize>() {
                        Ok(len) => {
                            let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
                            let body_string = String::from_utf8(body_bytes.to_vec())?;
                            let encoded = tokenizer.encode(body_string, false)?;
                            let mut trimmed = encoded.get_ids().to_vec();
                            trimmed.truncate(len);
                            let decoded = tokenizer.decode(trimmed,true).unwrap();
                            println!("Trimmed to {}",len);
                            Ok(Response::new(Body::from(decoded)))}
                        Err(_) => {
                            let mut not_found = Response::default();
                            *not_found.status_mut() = StatusCode::NOT_FOUND;
                            Ok(not_found)}
                    }
                }
                s if s.starts_with("/trimw/") => {
                    match s.strip_prefix("/trimw/").unwrap().parse::<usize>() {
                        Ok(len) => {
                            let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
                            let body_string = String::from_utf8(body_bytes.to_vec())?;
                            let encoded = tokenizer.encode(body_string, false)?;
                            let mut trimmed = encoded.get_ids().to_vec();
                            trimmed.truncate(len);
                            let decoded = tokenizer.decode(trimmed, true).unwrap();
                            match decoded.rsplit_once(" ")  {
                                None => {
                                    println!("Trimmed word to {}", decoded.len());
                                    Ok(Response::new(Body::from(decoded)))
                                }
                                Some((before,_after)) => {
                                    println!("Trimmed by word back from {}", decoded.len());
                                    Ok(Response::new(Body::from(before.to_string())))
                                }
                            }

                        }
                        Err(_) => {
                            let mut not_found = Response::default();
                            *not_found.status_mut() = StatusCode::NOT_FOUND;
                            Ok(not_found)}
                    }
                }
                _ => {
                    let mut not_found = Response::default();
                    *not_found.status_mut() = StatusCode::NOT_FOUND;
                    Ok(not_found)
                }
            }
        }

        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let server_details = env::var("SERVER_ADDR").unwrap_or("0.0.0.0:30000".to_string());
    let pretrained = env::var("PRETRAINED").unwrap_or("gpt2".to_string());

    let addr: SocketAddr = server_details
        .parse()
        .expect("Unable to parse socket address");

    let tokenizer = Tokenizer::from_pretrained(pretrained, None)?;
    let tokenizer = Arc::new(tokenizer);

    let service = make_service_fn(|_| {
        let svc_tokenizer = tokenizer.clone();
        async {
            Ok::<_, hyper::Error>(service_fn(move |b| {
                let fn_tokenizer = svc_tokenizer.clone();
                tokenize(fn_tokenizer, b)
            }))
        }
    });
    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}