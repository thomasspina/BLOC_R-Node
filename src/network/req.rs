use core::fmt;
use std::{error::Error, io::{Read, Write}, net::{SocketAddr, TcpStream}};
use rblock::Block;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use crate::GLOBAL_DB;

/// Enum to represent the status of responses
#[derive(PartialEq, Serialize, Deserialize)]
pub enum Status {
    /// everything is as expected
    OK,

    /// request did not meet expected format
    BadReq,

    /// received data was not valid
    BadData,

    /// There was an error internaly
    IntErr
}

// implement the display trait for status to print it more easily
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::OK => write!(f, "ok"),
            Status::BadReq => write!(f, "bad request"), 
            Status::BadData => write!(f, "bad data"),
            Status::IntErr => write!(f, "internal error")
        }
    }
}

/// Enum to represent the type of request/response
#[derive(PartialEq, Serialize, Deserialize)]
pub enum RType {
    /// Connect Test is used to verify a connection to a node
    ConnectTest,

    /// PushBlock is to push a newfound block to other nodes
    PushBlock,
}

/// Struct to represent the request
#[derive(Serialize, Deserialize)]
pub struct Request {
    pub req_type: RType,
    pub block: Option<Block>
}

/// Struct to represent the response
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub res_type: RType,
    pub status: Status
}

// TODO: add a function to request new blocks
// TODO: add function to request whole blockchain

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
    } else if req.req_type == RType::PushBlock {
        handle_push_block(stream, req)?;
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
        if res.status == Status::OK {
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
fn handle_connect_test(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // make response object
    let response: Response = Response {
        res_type: RType::ConnectTest,
        status: Status::OK
    };

    send_response(response, stream)?;

    Ok(())
}

fn handle_push_block(stream: TcpStream, req: Request) -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = stream.peer_addr()?;
    let mut response: Response = Response {
        res_type: RType::PushBlock,
        status: Status::OK
    };

    // check for bad request
    if req.block.is_none() {
        response.status = Status::BadReq;
        send_response(response, stream)?;
        return Err(format!("No block in push request from {}", addr).into());
    }

    let block: Block = req.block.unwrap();

    // preliminary block checks
    if block.verify_hash() || block.verify_transactions() || block.confirm_difficulty() {
        response.status = Status::BadData;
        send_response(response, stream)?;
        return Err(format!("Bad block data in push request from {}", addr).into());
    }

    // get global db for verifications
    match GLOBAL_DB.lock() {
        Ok(mut db) => {
            let latest_block = db.get_latest_block();

            // check if latest block exists in db
            if latest_block.is_err() {
                response.status = Status::IntErr;
                send_response(response, stream)?;
                return Err("Latest block doesn't exist in db".into());
            }

            let latest_block: Block = latest_block.unwrap();

            // check if block is latest
            if latest_block.get_height() > block.get_height() {
                response.status = Status::BadData;
                send_response(response, stream)?;
                return Err(format!("Latest block in req from {} is later than block from in db", addr).into());
            }

            if latest_block.get_height() + 1 < block.get_height() {
                // TODO: request other blocks coming up to it first
            }

            // TODO: verify difficulty
            // TODO: verify old block hash fits
            // TODO: verify transacations in db (with chainstate)
            // TODO: put block in db
        },
        
        // db is inaccessible
        Err(e) => {
            response.status = Status::IntErr;
            send_response(response, stream)?;
            return Err(e.into());
        }
    }

    // TODO: propagate block
    send_response(response, stream)?;
    Ok(())
}


/// Helper function to send a response to a client
/// 
/// # Arguments
/// * `response` - The response to send
/// * `stream` - The tcp stream on which to send the response
/// 
/// # Modifications
/// * Closes the stream after sending the response
/// 
/// # Returns
/// * `Result<(), Box<dyn Error>>` - The result of sending the response
/// 
fn send_response(response: Response, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
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