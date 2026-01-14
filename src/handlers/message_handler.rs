use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use crate::agents::agent::AgentName;

use crate::TunnelImpl;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub agent: String,
    pub message: String,
}

pub async fn message_handler(
    tunnel: web::Data<TunnelImpl>,
    body: web::Json<Message>,
) -> impl Responder {
    let agent = AgentName::from_string(body.agent.clone());

    match tunnel.send_to_agent(agent, body.message.clone()).await {
        Ok(_) => HttpResponse::Ok().body(format!("Message sent to {}", body.agent)),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}
