use crate::{indexer, tf_idf, types::TFI};
use std::{fs::File, io::Error, path::Path};
use tiny_http::{Header, Method, Request, Response, Server};

fn serve_static_file(request: Request, file_path: &str, content_type: &str) -> Result<(), Error> {
    let file = File::open(file_path)?;
    let header = Header::from_bytes("Content-Type", content_type).unwrap();
    let response = Response::from_file(file)
        .with_header(header)
        .with_status_code(200);
    request.respond(response)
}

fn search(mut request: Request, tf_index: &TFI) -> Result<(), Error> {
    let mut body = String::new();
    let _ = request.as_reader().read_to_string(&mut body);
    let ranks = tf_idf::compute_ranks(body, tf_index);
    let json = serde_json::to_value(ranks)?.to_string();
    let header = Header::from_bytes("Content-Type", "application/json").unwrap();
    request.respond(
        Response::from_string(json)
            .with_header(header)
            .with_status_code(201),
    )
}

pub(crate) fn serve(port: usize, index_path: &Path) -> anyhow::Result<()> {
    let server = Server::http(format!("0.0.0.0:{port}")).unwrap();

    println!("listening at {addr} ...", addr = server.server_addr());

    let tf_index = indexer::read_index(index_path)?;

    for request in server.incoming_requests() {
        match (request.method(), request.url()) {
            (Method::Post, "/api/search") => {
                let _ = search(request, &tf_index);
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
