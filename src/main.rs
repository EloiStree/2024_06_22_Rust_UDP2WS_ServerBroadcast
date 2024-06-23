
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use tungstenite::{accept, Message};
use uuid::Uuid;

use std::sync::{Arc, Mutex};



//https://crates.io/crates/tokio-tungstenite
//https://crates.io/crates/fastwebsockets
// https://crates.io/crates/tungstenite
// 


struct WebSocketRSATunnelHandshake <'handshake, 'websocket>{
    ref_handshake_state: &'handshake RSATunnelHanshake,
    ref_websocket: &'websocket tungstenite::WebSocket<TcpStream>,
}

struct RSATunnelHanshake{
     public_rsa_key_given : String,
     sent_guid : String,
     received_signed_guid_b64 : String,
     received_signed_guid : Vec<u8>,
     waiting_verification_to_compute:bool,
     server_was_able_to_verify_signed_guid : bool,
     is_signed_guid_valid : bool,
}



pub fn CheckIfSignedGuidIsValid(received_signed_guid_b64:String, public_rsa_key_given: String, sent_guid: String)-> bool {
    let trust_user_on_his_identity: bool = true;
    //To add later with RSAWSCALL

    return trust_user_on_his_identity;
}

/// A WebSocket echo server
fn main () {

    //let mut dictionnary_rsa_websocket_list: HashMap<String, vec<WebSocketRSATunnelHandshake>> = HashMap::new();
    //let mut handshake_list: Arc<Mutex<Vec<WebSocketRSATunnelHandshake>>> = Arc::new(Mutex::new(Vec::new()));

       
   
        
    let  use_print= true;
    let server = TcpListener::bind("0.0.0.0:4504").unwrap();
    for stream in server.incoming() {
        let ref_list = handshake_list.clone();
        spawn (move || {

            let mut websocket = accept(stream.unwrap()).unwrap();
       
            let mut client = RSATunnelHanshake {
                public_rsa_key_given: String::from(""),
                sent_guid: String::from(""),
                received_signed_guid_b64: String::from(""),
                received_signed_guid: vec![],
                waiting_verification_to_compute: false,
                server_was_able_to_verify_signed_guid: false,
                is_signed_guid_valid: false,
            };
            let  client_connection = WebSocketRSATunnelHandshake {
                ref_handshake_state: &client,
                ref_websocket: &websocket,
            };


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
                            
                            let is_valid =CheckIfSignedGuidIsValid(client.received_signed_guid_b64, client.public_rsa_key_given, client.sent_guid);
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
    }
}