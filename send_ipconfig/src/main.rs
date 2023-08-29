use actix::Actor;
use actix::StreamHandler;
use actix_http::body;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws::WebsocketContext;
use actix_web_actors::ws::{self, Message};
use env_logger::*;
use log::{error, info, Level};
use std::env;
use reqwest::Client;
use dotenv::dotenv;
use encoding_rs::{UTF_8, GBK};

#[actix_web::main]
async fn main() {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let host_ip = env::var("HOST_IP").expect("HOST_IP not set");
    info!("HOST_IP: {}", host_ip);
    let host_port = env::var("HOST_PORT").expect("HOST_PORT not set");
    info!("HOST_PORT: {}", host_port);
    loop{
        send_ipconfig(host_ip.clone(), host_port.clone()).await;
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

async fn get_ipconfig() -> String {
    let output = std::process::Command::new("ipconfig")
        .arg("/all")
        .output()
        .expect("failed to execute process");
    match output.status.success(){
        true => {
            let (result, _, _) = GBK.decode(&output.stdout);
            // info!("ipconfig: {}", result.to_string());
            result.to_string()
        },
        false => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            stderr.to_string()
        },
    }
}

async fn send_ipconfig(host_ip: String, host_port: String){
    let client = Client::new();
    let body = get_ipconfig().await;
    let res = client.post(format!("http://{}:{}/ipconfig", host_ip, host_port))
        .body(body)
        .send()
        .await;
    info!("send ipconfig to server: {:?}", res);
}