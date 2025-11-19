use rudis::args::Args;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    let args = Arc::new(Args::load());
    env_logger::init();
    service_into(args.clone());
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
