use std::{error::Error, net::TcpListener, thread};
use reqwest::Client;
use serde::Serialize;
use crate::network::req::handle_client_request;


#[derive(Serialize)]
struct Port(u16);


/// Start the node. 
/// This function will start the node on the given port and send a POST request to the seed server to notify it of the new node.
/// If the POST request fails, the function will return an error but won't stop the tcp server from running.
/// The node won't be recognized or visible to other nodes if the POST request fails.
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - The result of starting the node
/// 
pub async fn start_node(port: u16) -> Result<(), Box<dyn Error>> {
    let port: Port = Port(port);

    // async run environment to handle server verification request
    tokio::spawn(async move {
        // set up temporary server for initial seed server connection verification
        let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{}", port.0)).expect("Could not bind to port");
        println!("Node listening on port {}", port.0);
    
        // handle incoming connections
        for stream in listener.incoming() {

            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());

                    // Spawn a new thread for each connection.
                    thread::spawn(move || {
                        match handle_client_request(stream) {
                            Ok(_) => {},
                            Err(e) => println!("Error handling request: {}", e)
                        }
                    });
                },
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    });
    
    // POST req to seed server to tell him we're here
    let client: Client = reqwest::Client::new();
    let res: reqwest::Response = client.post(super::SEED_SERVER_ADDR)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&port).unwrap())
        .send()
        .await?;

    // status check, will be 500 if server is failing to handle requests
    if res.status() != 200 {
        return Err(format!("POST request FAILED on Seed Server. Status: {}\nCheck Firewall to allow traffic on port: {}", res.status(), port.0).into());
    }

    Ok(())
}

