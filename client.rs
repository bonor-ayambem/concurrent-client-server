use std::net::TcpStream;
use std::io::{Read, Write};
// use std::sync::mpsc::Sender;

// use crate::server::handle_requests;

/*
    req is passed to this function as a tuple of Strings.
    a get request will be ("get", "<key>")
    a del request will be ("del", "<key>")
    a put request will be ("<key>", "<value>")
*/
// make request an enum
pub fn send_request(req: (String, String), mut stream: TcpStream) {
    // let mut stream = TcpStream::connect(127.0.0.1:1895);
    // println!("Connected to server.");

    // build requeSst
    // let mut request = String::new();
    let request = match &req.0 as &str {
        "get" => {
            let key_len = req.1.len();
            format!("GET\nKEY-LEN: {}\n{}\n", key_len, req.1)
        }
        "del" => {
            let key_len = req.1.len();
            format!("DEL\nKEY-LEN: {}\n{}\n", key_len, req.1)
        }
        _ => {
            let key_len = req.0.len();
            let val_len = req.0.len();
            format!("PUT\nKEY-LEN: {}\n{}\nVAL-LEN: {}\n{}\n", key_len, req.0, val_len, req.1)
        }
    };

    // send to server
    if let Err(err) = stream.write(request.as_bytes()) {
        eprintln!("Error sending request: {}", err);
        return;
    }

    // let mut r = Vec::new();
    // r.push(b'h');
    // stream.read(&mut r);
    println!("here i am");

    // receive response
    let mut response = Vec::new();
    if let Err(err) = stream.read_to_end(&mut response) {
        eprintln!("Error reading response: {}", err);
        return;
    }
    
    // Check if the response is empty
    if response.is_empty() {
        eprintln!("Empty response received");
        return;
    }
    println!("Response received: {:?}", response);

    // let mut buffer = [0u8; 512];
    // let bytes_read = match stream.read(&mut buffer) {
    //     Ok(bytes_read) => bytes_read,
    //     Err(err) => {
    //         eprintln!("Error reading from socket: {}", err);
    //         return;
    //     }
    // };
    // let response_bytes = &buffer[..bytes_read];

    // let mut response = String::new();
    // let _ = stream.read_to_string(&mut response);



    // print out response
    if let Ok(response_str) = std::str::from_utf8(&response) {
        println!("Response: {}", response_str);

        // // Send the response back through the channel
        // if let Err(err) = response_sender.send(response_str.to_string()) {
        //     eprintln!("Error sending response through channel: {}", err);
        // }
    } else {
        eprintln!("Error decoding response as UTF-8");
        eprintln!("Response bytes: {:?}", response);

    }

    println!("Server response: {:?}", response);

    // println!("{:?}", response_bytes);
}