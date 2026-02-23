use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::{tungstenite::protocol::Message, accept_hdr_async};
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use futures::{SinkExt, StreamExt};
use tracing::{info, error, warn};
use gnunet_social::mqtt::MqttServer;
use gnunet_social::protocol::ServerMessage;

type Tx = broadcast::Sender<String>;

pub struct WebSocketServer {
    addr: SocketAddr,
    mqtt_server: Arc<MqttServer>,
    broadcast_tx: Tx,
}

impl WebSocketServer {
    pub fn new(addr: SocketAddr) -> Self {
        let mqtt_server = Arc::new(MqttServer::new());
        let (broadcast_tx, _) = broadcast::channel(1024);
        
        Self {
            addr,
            mqtt_server,
            broadcast_tx,
        }
    }
    
    pub fn mqtt_server(&self) -> Arc<MqttServer> {
        self.mqtt_server.clone()
    }
    
    pub async fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        info!("WebSocket server listening on {}", self.addr);
        
        let broadcast_tx = self.broadcast_tx.clone();
        let mqtt_server = self.mqtt_server.clone();
        
        while let Ok((stream, addr)) = listener.accept().await {
            let broadcast_tx = broadcast_tx.clone();
            let mqtt_server = mqtt_server.clone();
            let mut broadcast_rx = broadcast_tx.subscribe();
            
            tokio::spawn(async move {
                info!("New connection from {}", addr);
                
                let ws_stream = match accept_hdr_async(stream, |req: &Request, response: Response| {
                    let _path = req.uri().path();
                    Ok(response)
                }).await {
                    Ok(s) => s,
                    Err(e) => {
                        warn!("WebSocket handshake failed: {}", e);
                        return;
                    }
                };
                
                let (mut ws_sender, mut ws_receiver) = ws_stream.split();
                
                loop {
                    tokio::select! {
                        msg = ws_receiver.next() => {
                            match msg {
                                Some(Ok(Message::Text(text))) => {
                                    if let Some(response) = mqtt_server.process_message(text.as_bytes()) {
                                        let json = serde_json::to_string(&response).unwrap();
                                        if let Err(e) = ws_sender.send(Message::Text(json.into())).await {
                                            error!("Failed to send message: {}", e);
                                            break;
                                        }
                                        
                                        if let ServerMessage::Event(event) = response {
                                            let event_json = serde_json::to_string(&event).unwrap();
                                            let _ = broadcast_tx.send(event_json);
                                        }
                                    }
                                }
                                Some(Ok(Message::Binary(data))) => {
                                    if let Some(response) = mqtt_server.process_message(&data) {
                                        let json = serde_json::to_string(&response).unwrap();
                                        if let Err(e) = ws_sender.send(Message::Text(json.into())).await {
                                            error!("Failed to send message: {}", e);
                                            break;
                                        }
                                    }
                                }
                                Some(Ok(Message::Ping(data))) => {
                                    if let Err(e) = ws_sender.send(Message::Pong(data)).await {
                                        error!("Failed to send pong: {}", e);
                                        break;
                                    }
                                }
                                Some(Ok(Message::Close(_))) => {
                                    info!("Client {} disconnected", addr);
                                    break;
                                }
                                Some(Err(e)) => {
                                    error!("WebSocket error: {}", e);
                                    break;
                                }
                                None => break,
                                _ => {}
                            }
                        }
                        
                        event = broadcast_rx.recv() => {
                            match event {
                                Ok(json) => {
                                    if let Err(e) = ws_sender.send(Message::Text(json.into())).await {
                                        error!("Failed to broadcast: {}", e);
                                        break;
                                    }
                                }
                                Err(broadcast::error::RecvError::Closed) => break,
                                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                            }
                        }
                    }
                }
                
                info!("Connection closed for {}", addr);
            });
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let server = WebSocketServer::new(addr);
    
    info!("GNUnet Social Media Server starting...");
    info!("WebSocket endpoint: ws://{}", addr);
    
    server.run().await
}
