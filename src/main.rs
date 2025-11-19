#[tokio::main]
async fn main() {
    env_logger::init();
    service_into();
}

fn service_into() {
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
        "v0.1.0", 6379, 1, "master"
    );
    println!("{}", pattern);
}
