use hello::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let http_ok = "HTTP/1.1 200 OK";
    let http_404 = "HTTP/1.1 400 NOT FOUND";

    let (status_line, filename) = if buffer.starts_with(get) {
        (http_ok, "resources/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (http_ok, "resources/hello.html")
    } else {
        (http_404, "resources/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = match ThreadPool::build(4) {
        Ok(t_pool) => t_pool,
        Err(pool_err) => {
            println!("{}: Generating default pool of size 4", pool_err.msg);
            ThreadPool::new(4)
        }
    };

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
