use std::{error::Error, io::{Read, Write}, net::TcpStream};
use serde::{Serialize, Deserialize};
use std::time::Duration;


/// Enum to represent the type of request/response
#[derive(PartialEq, Serialize, Deserialize)]
pub enum RType {
    ConnectTest
}

/// Struct to represent the request
#[derive(Serialize, Deserialize)]
pub struct Request {
    pub req_type: RType
}

/// Struct to represent the response
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub res_type: RType,
    pub status: u8
}

/// Handles each tcp node connection. Each stream is handled as a seperate request.
/// A single response is sent for every request
/// 
/// # Arguments
/// * `stream` - The tcp stream on which the node is connected
/// 
/// # Returns
/// * `io::Result<()>` - The result of handling the client
/// 
pub fn handle_client_request(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut lenght_buf: [u8; 4] = [0u8; 4];

    // set timeout for reading from stream
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    stream.read_exact(&mut lenght_buf)?; // read_exact will read exactly 4 bytes

    // length of response buffer
    let length: usize = u32::from_le_bytes(lenght_buf) as usize;

    // buffer to hold response
    let mut buffer: Vec<u8> = vec![0u8; length];

    stream.read(&mut buffer)?;

    let req: Request = bincode::deserialize(&buffer)?;

    // handle request in accordance with its type
    if req.req_type == RType::ConnectTest {
        handle_connect_test(stream)?;
    }

    Ok(())
}

/// handle the response from the request. each request is handled as a seperate response
/// the responses are unique to the request type
/// 
/// # Arguments
/// * `stream` - The tcp stream on which the response is expected
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - The result of handling the response
/// 
pub fn handle_response(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // get length of buffer
    let mut lenght_buf: [u8; 4] = [0u8; 4];

    // set timeout for reading from stream
    stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;

    stream.read_exact(&mut lenght_buf)?; // read_exact will read exactly 4 bytes

    // length of response buffer
    let length: usize = u32::from_le_bytes(lenght_buf) as usize;

    // buffer to hold response
    let mut buffer: Vec<u8> = vec![0u8; length];

    stream.read(&mut buffer)?;

    let res: Response = bincode::deserialize(&buffer)?;

    if res.res_type == RType::ConnectTest {
        if res.status == 200 {
            return Ok(());
        } else {
            return Err(format!("ConnectTest failed. Status: {}", res.status).into());
        }
    }

    Ok(())
}   

/// Handles the connect test request type
/// This request is used to verify that the node is up and running
/// 
/// # Arguments
/// * `stream` - The tcp stream on which the test connection is made
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - The result of handling the client
/// 
fn handle_connect_test(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // make response object
    let response: Response = Response {
        res_type: RType::ConnectTest,
        status: 200
    };

    // serialize responses
    let bytes: Vec<u8> = bincode::serialize(&response)?;
    let buffer_size: [u8; 4] = (bytes.len() as u32).to_le_bytes();

    // send response
    stream.write_all(&buffer_size)?;
    stream.write_all(&bytes)?;

    // close stream
    drop(stream);

    Ok(())
}