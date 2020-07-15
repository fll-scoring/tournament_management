/*!
* This crate provides functionality for getting and setting information about teams and
* tournaments.
!*/
#![warn(missing_docs)]
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use log::info;
use std::env;

/// Data models
pub mod models;

/// Contains all routes for API services
pub mod routes;
use routes::*;

use fll_scoring::config::get_service_config_value;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let bind_addr = match get_service_config_value("tournament-management", "bind-addr", false) {
        Ok(addr) => addr,
        Err(_) => String::from("127.0.0.1:8000"),
    };
    // Initialize logging framework
    if let Err(err) = setup_logging() {
        eprintln!("Error setting up logging: {:?}", err);
    }
    info!("Starting HTTP server - listening on {}", &bind_addr);
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(new_team)
            .service(get_team)
            .service(new_tournament)
            .service(get_tournament)
    })
    .bind(bind_addr)?
    .run()
    .await
}

fn setup_logging() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
