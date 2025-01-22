use crate::indexer;
use crate::tf_idf;
use crate::types::Model;

use std::fs::File;
use std::io::Error;
use std::path::Path;
use tiny_http::Header;
use tiny_http::Method;
use tiny_http::Request;
use tiny_http::Response;
use tiny_http::Server;

fn serve_static_file(request: Request, file_path: &str, content_type: &str) -> Result<(), Error> {
    let file = File::open(file_path)?;
    let header = Header::from_bytes("Content-Type", content_type).unwrap();
    let response = Response::from_file(file)
        .with_header(header)
        .with_status_code(200);
    request.respond(response)
}

fn search(mut request: Request, model: &Model) -> Result<(), Error> {
    let mut body = String::new();
    let _ = request.as_reader().read_to_string(&mut body);
    let ranks = tf_idf::compute_scores(body, model);
    let json = serde_json::to_value(ranks)?.to_string();
    let header = Header::from_bytes("Content-Type", "application/json").unwrap();
    request.respond(
        Response::from_string(json)
            .with_header(header)
            .with_status_code(201),
    )
}

pub(crate) fn serve(port: usize, index_path: &Path) -> anyhow::Result<()> {
    let server = Server::http(format!("0.0.0.0:{port}")).expect("Server to start");

    println!("listening at {} ...", server.server_addr());

    let model = indexer::read_index(index_path)?;

    for request in server.incoming_requests() {
        match (request.method(), request.url()) {
            (Method::Post, "/api/search") => {
                let _ = search(request, &model);
            }
            (Method::Get, "/" | "/index.html") => {
                let _ = serve_static_file(request, "index.html", "text/html;charset=utf-8");
            }
            (Method::Get, "/index.js") => {
                let _ = serve_static_file(request, "index.js", "text/javascript;charset=utf-8");
            }
            _ => {
                let _ = serve_static_file(request, "404.html", "text/html;charset=utf-8");
            }
        }
    }

    Ok(())
}
