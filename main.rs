use homework_2::server::handle_requests;

fn main() {
    // Start the server
    if let Err(e) = handle_requests() {
        eprintln!("Error starting the server: {}", e);
    }
}

/* 
    test: four clients and one server

    client: builds requests and sends them to server
            receives response from SERVER and display results

    server: receives request from CLIENT, performs operation,
            and sends back response

    datastore: hashmap data structure - get, del, put
            *concurrent - every call to it is in a new thread?

    
    
    use the port 1895 for the network communication

    use sockets (net)


    */