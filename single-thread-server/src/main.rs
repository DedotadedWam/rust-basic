/*
--------------------> Building the Server <--------------------
bind acts as the keyword new it creates a new instantiation of the TCP server.
in networking the term bind means connecting to a port.
unwrap is a way for us to handle errors. The bind method returns a Results <T, E> translates to success/error. Unrwap will exit the program if there is an error.
the incoming mehtod in the TcpListener returns an iterator that gives us a sequence of streams.
Each stream represents an open connection between the cleint and the server, and connection refers to the request response cycle.

--------------------> Handling Requests <--------------------
use std::io::prelude::*; - get access to certain traits that let us read from and write to the stream
handle_connection requires a mutable stream as the input.
Why is it mutable?
The TcpStream instantiations keeps track of what data it returns internally. If it returns more data than is asked for, it holds onto that data for use if needed later. The internal state of TcpStream might change.
A buffer array is instantiated on the stack so that we can read out stream and write to buffer array. This is done via stream.read(buffer) which takes buffer as an argument to write to. Must be mutable and is borrowed so something can be done with the buffer once it is written to, such as println!-ing it.
--------------------> Sending Back a Response <--------------------
A response string is created and each /r/n separates the response from the header from the body.
stream.write takes a buffer as an arguement (type &[u8]) which is why the response is being converted to bytes (converts data to &[u8])
.flush() stops the code from executing until all of the bytes have been written to the connection
*/
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;


fn main() {
    // binds the provided url as the address for the TcpListener instance.
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    // .incoming() makes the listener an iterable where each HTTP request that comes in is a stream
    for stream in listener.incoming() {
        // call unwrap on the stream to handle any errors related to the incoming stream.
        let stream = stream.unwrap();

        println!("Connection Established");
        handle_connection(stream);
    }
}

fn handle_connection (mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_code, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}