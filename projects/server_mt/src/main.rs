
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:7878").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        println!("connected");
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) {
    let mut buf_read = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_read.read_line(&mut request_line).await.unwrap();

    println!("{request_line:#?}");

    let valid_pages = vec!["/index"];

    let mut request_page = request_line.split_whitespace().nth(1).unwrap_or("/");

    println!("{request_page:#?}");

    if request_page == "/" {
        request_page = "/index";
    }
    println!("{request_page:#?}");

    if valid_pages.contains(&request_page) {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(format!("static{request_page}.html")).await.unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).await.unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("static/404.html").await.unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).await.unwrap();
    }
}
