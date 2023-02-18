use tokio::{io, net::TcpListener};

pub struct ServerConfig<'a> {
    pub address: &'a str,
}

pub async fn run(config: &ServerConfig<'_>) -> io::Result<()> {
    let listener = TcpListener::bind(config.address).await?;
    println!("server running on {}", config.address);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("acception new socket");
                // TODO: spawn new action that executes a private function
                // which will read/write using the stream. Pass ownership
                // of the stream to the private function, since we don't need
                // it anymore.
            }
            Err(err) => {
                println!("failed to accept: {}", err);
            }
        }
    }
}
