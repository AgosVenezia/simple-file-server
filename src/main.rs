use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use infer;
use url_escape::decode;


fn handle_client(mut stream: TcpStream, root_dir: &Path) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();


    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = match request.lines().next() {
        Some(line) => line,
        None => {
            respond_with_400(&mut stream);
            return;
        }
    };


    let path = match request_line.split_whitespace().nth(1) {
        Some(p) => p,
        None => {
            respond_with_400(&mut stream);
            return;
        }
    };


    let decoded_path = decode(path).into_owned();
   
    // Serve a specific HTML file for a specific route
    if decoded_path == "/index.html" {
        let html_file_path = root_dir.join("index.html");
        respond_with_html_file(&mut stream, &html_file_path);
        return;
    }


    let resource_path = root_dir.join(decoded_path.trim_start_matches('/'));


    if !resource_path.starts_with(root_dir) {
        // Prevent backtracking outside the root directory
        respond_with_404(&mut stream);
        return;
    }


    if resource_path.is_dir() {
        respond_with_directory_listing(&mut stream, &resource_path);
    } else if resource_path.is_file() {
        respond_with_file(&mut stream, &resource_path);
    } else {
        respond_with_404(&mut stream);
    }
}


fn respond_with_directory_listing(stream: &mut TcpStream, dir: &Path) {
    let mut html_body = r#"<!DOCTYPE html><html><head><meta charset="utf-8"></head><body>"#.to_string();
    html_body.push_str(&format!("<h1>Directory listing for {}</h1><ul>", dir.display()));


    for entry in WalkDir::new(dir).max_depth(1).min_depth(1) {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy();
            html_body.push_str(&format!(
                r#"<li><a href="/{}">{}</a></li>"#,
                file_name, file_name
            ));
        }
    }


    html_body.push_str("</ul></body></html>");


    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        html_body.len(),
        html_body
    );
    stream.write_all(response.as_bytes()).unwrap();
}


fn respond_with_file(stream: &mut TcpStream, file_path: &Path) {
    let mime_type = infer::get_from_path(file_path)
        .ok()
        .and_then(|res| res)
        .map_or("application/octet-stream", |t| t.mime_type());


    let content = fs::read(file_path).unwrap();


    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        content.len(),
        mime_type
    );


    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&content).unwrap();
}


fn respond_with_html_file(stream: &mut TcpStream, file_path: &Path) {
    if !file_path.exists() {
        respond_with_404(stream);
        return;
    }


    let content = fs::read_to_string(file_path).unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        content.len(),
        content
    );
    stream.write_all(response.as_bytes()).unwrap();
}


fn respond_with_404(stream: &mut TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 13\r\nContent-Type: text/plain\r\n\r\n404 Not Found";
    stream.write_all(response.as_bytes()).unwrap();
}


fn respond_with_400(stream: &mut TcpStream) {
    let response = "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: 11\r\nContent-Type: text/plain\r\n\r\nBad Request";
    stream.write_all(response.as_bytes()).unwrap();
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let root_dir = std::env::current_dir().unwrap();


    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream, &root_dir);
    }
}
