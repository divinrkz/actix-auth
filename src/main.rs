mod config;

use color_eyre::Result;
use crate::config::Config;
use actix_web::{App, HttpServer, middleware::Logger};

#[actix_rt::main]
async fn main() -> Result<()> {
    
    let config: Config = Config::from_env() .expect("Server configuration");

     HttpServer::new(move || {
         App::new()
                .wrap(Logger::default())
     })   
     .bind(format!("{}:{}", config.HOST, config.PORT))?
     .run()
     .await?;

     Ok(())

}
