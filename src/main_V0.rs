
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use std::vec;
use tungstenite::{accept, Message};
use uuid::Uuid;

use std::sync::{Arc, Mutex};
use std::clone::Clone;



//https://crates.io/crates/tokio-tungstenite
//https://crates.io/crates/fastwebsockets
// https://crates.io/crates/tungstenite
// 


struct WebSocketRSATunnelHandshakeRef <'handshake, 'websocket>{
    ref_handshake_state: &'handshake RSATunnelHanshake,
    ref_websocket: &'websocket tungstenite::WebSocket<TcpStream>,
    ref_given_index: u32
}

struct WebSocketRSATunnelHandshake {
    handshake_state: RSATunnelHanshake,
    websocket:  tungstenite::WebSocket<TcpStream>,
    given_index: u32
}

struct RSATunnelHanshake{
     u32_connection_index_id: u32,
     public_rsa_key_given : String,
     sent_guid : String,
     received_signed_guid_b64 : String,
     received_signed_guid : Vec<u8>,
     waiting_verification_to_compute:bool,
     server_was_able_to_verify_signed_guid : bool,
     is_signed_guid_valid : bool,
}



impl Clone for RSATunnelHanshake {
    fn clone(&self) -> Self {
        RSATunnelHanshake {
            u32_connection_index_id: self.u32_connection_index_id,
            public_rsa_key_given: self.public_rsa_key_given.clone(),
            sent_guid: self.sent_guid.clone(),
            received_signed_guid_b64: self.received_signed_guid_b64.clone(),
            received_signed_guid: self.received_signed_guid.clone(),
            waiting_verification_to_compute: self.waiting_verification_to_compute,
            server_was_able_to_verify_signed_guid: self.server_was_able_to_verify_signed_guid,
            is_signed_guid_valid: self.is_signed_guid_valid,
        }
    }
}



pub fn CheckIfSignedGuidIsValid(received_signed_guid_b64:String, public_rsa_key_given: String, sent_guid: String)-> bool {
    let trust_user_on_his_identity: bool = true;
    //To add later with RSAWSCALL

    return trust_user_on_his_identity;
}

/// A WebSocket echo server
fn main_backup () {

    //let mut dictionnary_rsa_websocket_list: HashMap<String, vec<WebSocketRSATunnelHandshake>> = HashMap::new();
    let mut handshake_list: Arc<Mutex<Vec<RSATunnelHanshake>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handshake_connection_list: Arc<Mutex<Vec<WebSocketRSATunnelHandshakeRef>>> = Arc::new(Mutex::new(Vec::new()));
    let mut connection_index:u32=0;

    let mut created_websocket_list: HashMap<u32, &tungstenite::WebSocket<TcpStream>> = HashMap::new();

    let  use_print= true;
    let server = TcpListener::bind("0.0.0.0:4504").unwrap();



    for stream in server.incoming() {
        connection_index+=1;
        println!("New connection: {}", connection_index);
        let mut websocket = accept(stream.unwrap()).unwrap();
        
        let mut client = RSATunnelHanshake {
            u32_connection_index_id: connection_index,
            public_rsa_key_given: "".to_string(),
            sent_guid: "".to_string(),
            received_signed_guid_b64: "".to_string(),
            received_signed_guid: Vec::new(),
            waiting_verification_to_compute: false,
            server_was_able_to_verify_signed_guid: false,
            is_signed_guid_valid: false,
        };

        println!("Start Listening to braodcast channel: {}", connection_index);
        spawn ({
            let mut rx= tx.subscribe();
            loop {

                let msg = rx.recv().unwrap();
                websocket.send(Message::Text(msg)).unwrap();

            }

        });
        println("Start Listening to websocket: {}", connection_index);
        //let ref_list = handshake_list.clone();
        spawn (move || {

            println!("Thread of on index: {}", connection_index);
            loop {
                
                let msg = websocket.read().unwrap();
                
                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    if use_print {
                        if msg.is_binary() {
                            println!("Received: {:?}", msg.len() as u32);
                        } 
                        if msg.is_text(){
                            
                            println!("Received: {}", msg.to_text().unwrap());
                        }
                    }
                    if msg.is_text()
                    {
                        let text_msg = msg.to_text().unwrap();
                        if text_msg.starts_with("Hello ") {
                            let  key = &text_msg[6..].replace("\n", "").replace("\r", "").replace(" ", "");
                            client.public_rsa_key_given = key.to_string();
                            client.sent_guid = Uuid::new_v4().to_string();
                            let signin: String   = format!("SIGNIN:{}",client.sent_guid);
                            websocket.send(Message::Text(signin.to_string())).unwrap();


                        }
                        if text_msg.starts_with("SIGNED:") {
                            let signed = &text_msg[7..];
                            client.received_signed_guid_b64 = signed.to_string();
                            client.received_signed_guid = base64::decode(signed).unwrap();
                            
                            let is_valid =CheckIfSignedGuidIsValid(
                                client.received_signed_guid_b64,
                                 client.public_rsa_key_given.to_string(),
                                  client.sent_guid.to_string()
                                );
                            client.is_signed_guid_valid = is_valid;

                            if client.is_signed_guid_valid
                            {
                                websocket.send(Message::Text("RSA not supported yet on server. You are verified by default for now.\n Feature will be added as soon as possible.".to_string())).unwrap();
                                websocket.send(Message::Text("RSA:Verified".to_string())).unwrap();
                                websocket.send(Message::Text("IndexLock:None".to_string())).unwrap();
                                println!("User Valided by server.");
                            }
                            else
                            {
                                websocket.send(Message::Text("RSA:NotVerified".to_string())).unwrap();
                                websocket.close(None).unwrap();
                                println!("User not Valided by server.");
                            }
                        }
                    }
                    websocket.send(msg).unwrap();
                }
            }
        });
        
        println!("TTTTTTTTTTTTTTTEEEEEEEEEESSSSSSSSTTTTTTT: {}","Hello World!");
    }
}