use actix_web::{App, HttpServer, web};
use grpc_server::{TunnelImpl, handlers, tunnel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_addr: std::net::SocketAddr = "127.0.0.1:50051".parse()?;
    let http_addr = "127.0.0.1:8080";

    let tunnel_for_grpc = TunnelImpl::new();
    let tunnel_for_http = tunnel_for_grpc.clone();

    println!("Starting gRPC server on {}", grpc_addr);

    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(tunnel::tunnel_service_server::TunnelServiceServer::new(
                tunnel_for_grpc,
            ))
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    println!("Starting Http server on {}", http_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tunnel_for_http.clone()))
            .route(
                "/api/health-check",
                web::get().to(handlers::health_check::health_check),
            )
            .route(
                "/api/message",
                web::post().to(handlers::message_handler::message_handler),
            )
    })
    .bind(http_addr)?
    .run()
    .await?;

    Ok(())
}
