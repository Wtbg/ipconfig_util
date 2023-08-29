use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use log::{error, info, Level};
use std::sync::Mutex;
use std::{env, fs::File, io::Write, path::PathBuf, process::Command};
lazy_static! {
    pub static ref IPCONFIG_OUTPUT: Mutex<String> = Mutex::new(String::new());
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "error");
    env_logger::init();
    HttpServer::new(|| App::new().service(ipconfig).service(get_ipconfig))
        .bind("0.0.0.0:7777")?
        .run()
        .await
}

//accept string as request body, save it in a local file ipconfig.txt
#[post("/ipconfig")]
async fn ipconfig(req_body: String) -> impl Responder {
    let mut file = std::fs::File::create("ipconfig.txt").unwrap();
    let mut ipconfig_output = IPCONFIG_OUTPUT.lock().unwrap();
    *ipconfig_output = req_body.clone();
    match file.write_all(req_body.as_bytes()) {
        Ok(_) => info!("write to file success"),
        Err(e) => error!("write to file failed: {}", e),
    }
    HttpResponse::Ok().body("ok")
}

#[get("/ipconfig")]
async fn get_ipconfig() -> impl Responder {
    let ipconfig_output = IPCONFIG_OUTPUT.lock().unwrap();
    info!("ipconfig: {}", ipconfig_output.clone());
    HttpResponse::Ok().body(ipconfig_output.clone())
}
