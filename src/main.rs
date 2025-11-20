use rudis::{args::Args, server::Server};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    let args = Arc::new(Args::load());

    unsafe {
        std::env::set_var("RUST_LOG", &args.loglevel);
    }
    env_logger::init();

    service_into(args.clone());
    let server = Server::new(args.clone());
    server.start().await;
}

fn service_into(args: Arc<Args>) {
    let version = env!("CARGO_PKG_VERSION");
    let role = if args.is_slave() { "slave" } else { "master" };
    let pattern = format!(
        r#"
         /\_____/\
        /  o   o  \          Rudis {}
       ( ==  ^  == )
        )         (          Bind: {} PID: {}
       (           )
      ( (  )   (  ) )        Role: {}
     (__(__)___(__)__)

    Rudis is a high-performance in memory database.
    "#,
        version, args.port, 1, role
    );
    println!("{}", pattern);
}
