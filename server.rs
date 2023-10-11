use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use std::sync::Arc;
use std::time::{Duration, Instant};


use crate::kv_store::ConcurrentHashMap;

const EXPECTED_GET: &[u8] = &[71, 69, 84]; // "GET" in ASCII
const EXPECTED_DEL: &[u8] = &[68, 69, 76]; // "DEL" in ASCII
const EXPECTED_PUT: &[u8] = &[80, 85, 84]; // "PUT" in ASCII

pub fn handle_requests() -> std::io::Result<()> {
    let map = Arc::new(ConcurrentHashMap::new());
    // let start_time = Instant::now();

    let listener = TcpListener::bind("127.0.0.1:1895")?;
    println!("Server listening on port 1895...");

     // Spawn a new thread to handle connections
    let client_connections = thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let map = Arc::clone(&map);

                    // Spawn a new thread to handle each client
                    thread::spawn(move || { // timer and joinhandle after
                        let mut start_time = Instant::now(); // for testing and benching
                        let mut request_count = 0;

                        loop { // loop to receive each message from the current client (stream)
                            let mut buffer = [0; 512];
    
                            match stream.read(&mut buffer) {
                                Ok(bytes_read) => {
                                    if bytes_read == 0 {
                                        break;
                                    }
    
                                    let received_data = &buffer[0..bytes_read];
                                    println!("{:?}", received_data);
                                    
                                    match &received_data[0..3] {
                                        EXPECTED_GET => {
                                            println!("Handling GET Request");
                                            let key_len_start = 13; // Start of the key length
                                            let mut key_len_end = 0;

                                            // Find the position of the second '\n'
                                            let mut count = 0;
                                            for (i, &c) in received_data.iter().enumerate() { 
                                                if c == b'\n' {
                                                    count += 1;
                                                    if count == 2 {
                                                        key_len_end = i;
                                                        break;
                                                    }
                                                }
                                            }

                                            let key_len_str = String::from_utf8_lossy(&received_data[key_len_start..key_len_end]).to_string();

                                            if let Ok(key_len) = key_len_str.parse::<usize>() {
                                                let key_start = key_len_end + 1; // Start of the key
                                                let key_end = key_start + key_len;

                                                let key = &received_data[key_start..key_end];
                                                let key_str = String::from_utf8_lossy(key);
                                                // Handle the GET request

                                                let response = match map.get(&key_str) {
                                                    Some(resp) => {
                                                        let key_len = resp.len();
                                                        format!("OK\nRESULT-LEN: {}\n{}\n", key_len, resp)
                                                    }
                                                    None => {
                                                        format!("ERROR\n")
                                                    }
                                                };

                                                stream.write_all(response.as_bytes());
                                                stream.flush();
                                            }
                                            else {
                                                println!("Error getting key length");
                                            }                                          
                                        }
                                        EXPECTED_DEL => {
                                            println!("Handling DEL Request");
                                            let key_len_start = 13; // Start of the key length
                                            let mut key_len_end = 0;

                                            // Find the position of the second '\n'
                                            let mut count = 0;
                                            for (i, &c) in received_data.iter().enumerate() { 
                                                if c == b'\n' {
                                                    count += 1;
                                                    if count == 2 {
                                                        key_len_end = i;
                                                        break;
                                                    }
                                                }
                                            }

                                            let key_len_str = String::from_utf8_lossy(&received_data[key_len_start..key_len_end]).to_string();

                                            if let Ok(key_len) = key_len_str.parse::<usize>() {
                                                let key_start = key_len_end + 1; // Start of the key
                                                let key_end = key_start + key_len;

                                                let key = &received_data[key_start..key_end];
                                                let key_str = String::from_utf8_lossy(key);

                                                let response = match map.del(key_str.to_string()) {
                                                    true => {
                                                        format!("OK\n")
                                                    }
                                                    false => {
                                                        format!("ERROR\n")
                                                    }
                                                };

                                                stream.write_all(response.as_bytes());
                                                stream.flush();
                                            }
                                            else {
                                                println!("Error getting key length");
                                            } 
                                        }
                                        EXPECTED_PUT => {
                                            println!("Handling PUT Request");
                                            let key_len_start = 13; // Start of the key length
                                            let mut key_len_end = 0;

                                            // Find the position of the second '\n'
                                            let mut count = 0;
                                            for (i, &c) in received_data.iter().enumerate() {
                                                if c == b'\n' {
                                                    count += 1;
                                                    if count == 2 {
                                                        key_len_end = i;
                                                        break;
                                                    }
                                                }
                                            }

                                            let key_len_str = String::from_utf8_lossy(&received_data[key_len_start..key_len_end]).to_string();

                                            if let Ok(key_len) = key_len_str.parse::<usize>() {
                                                let key_start = key_len_end + 1; // Start of the key
                                                let key_end = key_start + key_len;

                                                let key = &received_data[key_start..key_end];
                                                let key_str = String::from_utf8_lossy(key);

                                                let val_len_start = key_end + 1; // Start of the value length
                                                let mut val_len_end = val_len_start;

                                                // Find the position of the second '\n' for value length
                                                let mut count = 0;
                                                for (i, &c) in received_data[val_len_start..].iter().enumerate() {
                                                    if c == b'\n' {
                                                        count += 1;
                                                        if count == 2 {
                                                            val_len_end = val_len_start + i;
                                                            break;
                                                        }
                                                    }
                                                }

                                                let val_len_str = String::from_utf8_lossy(&received_data[val_len_start..val_len_end]).to_string();

                                                if let Ok(val_len) = val_len_str.parse::<usize>() {
                                                    let val_start = val_len_end + 1; // Start of the value
                                                    let val_end = val_start + val_len;

                                                    let value = &received_data[val_start..val_end];
                                                    let value_str = String::from_utf8_lossy(value);

                                                    // Handle the PUT request by inserting the key-value pair into the map
                                                    map.put(key_str.to_string(), value_str.to_string());

                                                    let response = format!("OK\n");

                                                    stream.write_all(response.as_bytes());
                                                    stream.flush();
                                                } else {
                                                    println!("Error getting value length");
                                                }
                                            } else {
                                                println!("Error getting key length");
                                            }
                                        }
                                        _ => {
                                            println!("Invalid request format");
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error reading from socket: {}", e);
                                    break;
                                }
                            }

                            // Increment the request count for each request processed
                            request_count += 1;

                            // Check if one second has elapsed
                            if start_time.elapsed() >= Duration::from_secs(1) {
                                // Print the request count and reset the timer and count
                                println!("Requests processed in one second: {}", request_count);
                                start_time = Instant::now();
                                request_count = 0;
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    });

    client_connections.join().unwrap();
    // // Keep the main thread alive
    // loop {
    //     std::thread::park();
    // }

    Ok(())
}