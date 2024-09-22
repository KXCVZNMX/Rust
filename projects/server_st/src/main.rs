use std::net::TcpListener;
use std::{
    fs, io::{prelude::*, BufReader}, net::TcpStream
};

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connected");
        handle_connection(stream);
   }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_read = BufReader::new(&mut stream);
    let request_line = buf_read.lines().next().unwrap().unwrap();

    println!("{request_line:#?}");

    let valid_pages = vec!["/index"];

    let mut request_page = &request_line.split_whitespace().nth(1).unwrap();

    println!("{request_page:#?}");

    if request_page == &"/" {
        request_page = &"/index";
    }
    println!("{request_page:#?}");

    if valid_pages.contains(request_page) {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(format!("static{request_page}.html")).unwrap();
        let length = contents.len();

        let response = 
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("static/404.html").unwrap();
        let length = contents.len();

        let response = 
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
        stream.write_all(response.as_bytes()).unwrap();
    }
}