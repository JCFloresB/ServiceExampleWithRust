use std::sync::atomic::AtomicI16;
use std::sync::Arc;

use actix_web::{web, App, HttpServer};

mod health;
mod repository;
mod user;
mod v1;
use repository::MemoryRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Inicio de servidor");
    let thread_counter = Arc::new(AtomicI16::new(1));
    //building shared state
    let repo = web::Data::new(MemoryRepository::default());
    // let repo = RepositoryInjector::new(MemoryRepository::default());
    // let repo = RepositoryInjector::new_shared(MemoryRepository::default());

    // init env vars
    dotenv::dotenv().ok();
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }
    // get PORT value of .env file and bilding url conextion
    let port = std::env::var("PORT").unwrap_or(String::from("8080"));
    let address = format!("127.0.0.1:{}", port);
    //starter the server
    HttpServer::new(move || {
        let thread_index =
            thread_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("Starting thread: {}", thread_index);
        //create App and starter services
        App::new()
            .app_data(web::Data::new(thread_index))
            .app_data(repo.clone())
            .configure(health::service)
            .configure(v1::service::<MemoryRepository>) //se hace por medio de una fachada para obtener el usuario
                                                        // .service(getuser)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!(
            "🔥🔥🔥 couldn't start the server in port {}: {:?}",
            port, err
        )
    })
    .run()
    .await
}
