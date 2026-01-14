pub mod agents;
pub mod handlers;

pub mod tunnel {
    tonic::include_proto!("tunnel.v1");
}

use agents::agent::AgentName;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use tunnel::tunnel_service_server::TunnelService;
use tunnel::{AgentMessage, ServerMessage};

pub type AgentSender = mpsc::Sender<Result<ServerMessage, tonic::Status>>;
pub type AgentsMap = Arc<RwLock<HashMap<AgentName, AgentSender>>>;

#[derive(Clone, Default)]
pub struct TunnelImpl {
    pub agents: AgentsMap,
}

impl TunnelImpl {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn send_to_agent(&self, agent: AgentName, message: String) -> Result<(), String> {
        let agents = self.agents.read().await;

        if let Some(tx) = agents.get(&agent) {
            tx.send(Ok(ServerMessage { message: message }))
                .await
                .map_err(|e| format!("Failed to send: {}", e))?;
            Ok(())
        } else {
            Err(format!("Agent: {} not found.", agent))
        }
    }
}

#[tonic::async_trait]
impl TunnelService for TunnelImpl {
    type ConnectTunnelStream = ReceiverStream<Result<ServerMessage, tonic::Status>>;

    async fn connect_tunnel(
        &self,
        request: tonic::Request<tonic::Streaming<AgentMessage>>,
    ) -> std::result::Result<tonic::Response<Self::ConnectTunnelStream>, tonic::Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        let agents = self.agents.clone();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(agent_msg) => {
                        if let Some(payload) = agent_msg.payload {
                            use tunnel::agent_message::Payload;

                            match payload {
                                Payload::Register(info) => {
                                    let _ = agents.write().await.insert(
                                        AgentName::from_string(info.agent_name.clone()),
                                        tx_clone.clone(),
                                    );

                                    println!("Agent registered: {}", info.agent_name);
                                    let welcome_msg = ServerMessage {
                                        message: format!("Welcome, {}!", info.agent_name),
                                    };

                                    if tx_clone.send(Ok(welcome_msg)).await.is_err() {
                                        println!("Failed to send welcome message");
                                        break;
                                    }
                                }
                                Payload::Response(resp) => {
                                    println!("Agent response: {}", resp.message);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Stream error: {}", e);
                        break;
                    }
                }
            }
            println!("Agent disconnected");
        });

        Ok(tonic::Response::new(ReceiverStream::new(rx)))
    }
}
