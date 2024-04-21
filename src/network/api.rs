use std::{error::Error, io::{self, Read}, net};
use tiny_http::Server;
use tokio::task::JoinHandle;

/// Start the node
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - The result of starting the node
/// 
pub async fn start_node(port: u16) -> Result<(), Box<dyn Error>> {

    // async run environment to handle server verification request
    let temp_server_handle: JoinHandle<()> = tokio::spawn(async move {
        // set up temporary server for initial seed server connection verification
        let temp_server: Server = Server::http(format!("0.0.0.0:{}", port)).unwrap();
        for req in temp_server.incoming_requests() {

            // TODO: handle server handshake
            println!("\nRequests. {}\n{}", req.method(), req.url());
        }
    });
    
    let client = reqwest::Client::new();
    let res = client.post("127.0.0.1/post")
        .body("data to send")
        .send()
        .await?;

    println!("POST request sent. Status: {}", res.status());
    println!("Response Headers:\n{:?}", res.headers());

    temp_server_handle.abort();

    println!("abort succes");

    // make http listener on same port
    // wait for server ping,
    // send back ok, 
    // delete listener

    // let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Could not bind to port");
    // println!("Node listening on port {}", port);
    
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             println!("New connection: {}", stream.peer_addr().unwrap());
    //             // Spawn a new thread for each connection.
    //             thread::spawn(move || {
    //                 let _  = handle_client(stream);
    //             });
    //         }
    //         Err(e) => {
    //             println!("Connection failed: {}", e);
    //         }
    //     }
    // }


    // make a post req to the webserver
    // if post was success (meaning that the server was able to talk to node on port ) make get
    
    // with list of nodes, in possession make greetings to nodes and get info
    // update db with info
    // keep listening on port for additional traffic
    // once in a while hit up the webserver for an updated list of available nodes
    //
    Ok(())
}


/// Handles each tcp node connection
/// 
/// # Arguments
/// * `stream` - The tcp stream on which the node is connected
/// 
/// # Returns
/// * `io::Result<()>` - The result of handling the client
/// 
pub fn handle_client(mut stream: net::TcpStream) -> io::Result<()> {
    // get length of buffer
    let mut lenght_buf: [u8; 4] = [0u8; 4];
    stream.read_exact(&mut lenght_buf)?; // read_exact will read exactly 4 bytes

    let length: usize = u32::from_le_bytes(lenght_buf) as usize;

    let mut buffer: Vec<u8> = vec![0u8; length];

    match stream.read(&mut buffer) {
        Ok(size) => {
            // Convert the buffer into a String and print it.
            let received_data = String::from_utf8_lossy(&buffer[..size]);
            println!("Received: {}", received_data);
        },
        Err(e) => println!("Failed to receive data: {}", e),
    }

    Ok(())
}