// use std::net::{ UdpSocket};
// use tokio::net::{TcpListener, TcpStream};
// use tokio::sync::broadcast;
// use tokio::time::{self, Duration};
// use websocket::{server, Message, OwnedMessage};

// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("0.0.0.0:4504").await.unwrap();
//     let (tx, _rx) = broadcast::channel::<String>(10); // Broadcast channel with capacity 10

//     // UDP socket setup
//     let udp_socket = UdpSocket::bind("127.0.0.1:7399").expect("Failed to bind to UDP socket");

//     // Spawn task to handle UDP packets
//     let udp_task = tokio::spawn(handle_udp_packets(udp_socket, tx.clone()));

   

//     // Accept incoming connections
//     while let Ok((stream, _)) = listener.accept().await {
//         tokio::spawn(handle_connection(stream, tx.subscribe()));
//     }

//     // Await UDP task and broadcast task to complete
//     let _ = udp_task.await;
// }

// async fn handle_connection(stream: TcpStream, mut rx: broadcast::Receiver<String>) {
//     let server = server::new(stream);
//     let mut client = match server.accept().await {
//         Ok(client) => client,
//         Err(_) => return,
//     };

//     // Listen for messages from the client
//     while let Ok(msg) = client.recv_message().await {
//         match msg {
//             OwnedMessage::Text(_) => {}, // Handle incoming client messages if needed
//             _ => continue,
//         }
//     }

//     // Listen for broadcast messages and send them to the client
//     while let Ok(msg) = rx.recv().await {
//         if let Err(_) = client.send_message(&Message::text(msg)).await {
//             break; // Break the loop if sending fails
//         }
//     }
// }

// async fn handle_udp_packets(socket: UdpSocket, tx: broadcast::Sender<String>) {
//     let mut buf = [0u8; 1024];

//     while let Ok((size, _src_addr)) = socket.recv_from(&mut buf).await {
//         let message = &buf[..size];
//         if let Ok(msg_str) = std::str::from_utf8(message) {
//             // Broadcast the received UDP message to all WebSocket clients
//             if let Err(_) = tx.send(msg_str.to_string()) {
//                 break; // Break the loop if sending fails
//             }
//         }
//     }
// }
