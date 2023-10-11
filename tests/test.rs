use homework_2::client::send_request;
// use homework_2::server::handle_requests;

use std::net::TcpStream;
// use std::sync::mpsc;
// use std::thread;

#[test]
fn test_client1() {
    // BENCH: how many requests can be sent in a second?
    // handle_requests();
    let client1_stream = TcpStream::connect("127.0.0.1:1895").expect("Failed to connect to server");

    send_request(("get".to_string(), "5".to_string()), client1_stream.try_clone().unwrap());
    send_request(("del".to_string(), "5".to_string()), client1_stream.try_clone().unwrap());
    send_request(("5".to_string(), "10".to_string()), client1_stream.try_clone().unwrap());
    send_request(("get".to_string(), "5".to_string()), client1_stream.try_clone().unwrap());
    send_request(("del".to_string(), "5".to_string()), client1_stream.try_clone().unwrap()); 
    send_request(("del".to_string(), "5".to_string()), client1_stream.try_clone().unwrap()); 
    send_request(("5".to_string(), "60".to_string()), client1_stream.try_clone().unwrap());
    send_request(("5".to_string(), "99".to_string()), client1_stream.try_clone().unwrap()); 

    send_request(("6".to_string(), "77".to_string()), client1_stream.try_clone().unwrap());
    send_request(("7".to_string(), "80".to_string()), client1_stream.try_clone().unwrap());
    send_request(("8".to_string(), "55".to_string()), client1_stream.try_clone().unwrap());
    send_request(("9".to_string(), "43".to_string()), client1_stream.try_clone().unwrap()); 
}

#[test]
fn test_client2() {
    let client2_stream = TcpStream::connect("127.0.0.1:1895").expect("Failed to connect to server");

    send_request(("get".to_string(), "5".to_string()), client2_stream.try_clone().unwrap());
    send_request(("del".to_string(), "5".to_string()), client2_stream.try_clone().unwrap());
    send_request(("5".to_string(), "10".to_string()), client2_stream.try_clone().unwrap());
    send_request(("get".to_string(), "5".to_string()), client2_stream.try_clone().unwrap());
    send_request(("del".to_string(), "5".to_string()), client2_stream.try_clone().unwrap()); 
    send_request(("del".to_string(), "5".to_string()), client2_stream.try_clone().unwrap()); 
    send_request(("5".to_string(), "60".to_string()), client2_stream.try_clone().unwrap());
    send_request(("5".to_string(), "99".to_string()), client2_stream.try_clone().unwrap()); 

    send_request(("6".to_string(), "77".to_string()), client2_stream.try_clone().unwrap());
    send_request(("7".to_string(), "80".to_string()), client2_stream.try_clone().unwrap());
    send_request(("8".to_string(), "55".to_string()), client2_stream.try_clone().unwrap());
    send_request(("9".to_string(), "43".to_string()), client2_stream.try_clone().unwrap()); 
}

// fn test_clients() {
//     // BENCH: how many requests can be sent in a second?
//     let client1_stream = TcpStream::connect("127.0.0.1:1895").expect("Failed to connect to server");

//     // Use a channel to capture the response
//     let (response_sender, response_receiver) = mpsc::channel();

//     thread::spawn(move || {
//         send_request(("get".to_string(), "5".to_string()), client1_stream.try_clone().unwrap(), response_sender);
//         // Assuming the send_request function sends a response back to the channel
//         // response_sender.send(response).unwrap();
//     });

//     // You can receive the response here
//     let response_str: String = response_receiver.recv().unwrap();
//     println!("Received response from client: {:?}", response_str);
// }

// does each client need to be able to send multiple messages?
// if yes: pass client stream as a parameter -> proving difficult

// if no: send_request will create a new client stream each time
// a client can only send one request... feels incorrect


// benchmarking?
// hashmap... recommendation