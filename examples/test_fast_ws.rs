// use moonsock::MoonConnection;
use std::env;

use moonsock::{
	// fast_ws_stuff::connect,
	// FastMoonConn,
	MoonMethod,
	MoonRequest,
	// MoonrakerClient,
	// moonraker_client_new::MoonrakerClient,
	MoonrakerClient,
};

const DEFAULT_MOONRAKER_PORT: u16 = 7125;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let hostname = env::var("MOONRAKER_HOSTNAME").expect("Please add the `MOONRAKER_HOSTNAME` environment variable");
	let port = match env::var("MOONRAKER_PORT") {
		Ok(port_string) => match port_string.parse::<u16>() {
			Ok(port) => port,
			Err(_e) => DEFAULT_MOONRAKER_PORT,
		},
		Err(_e) => DEFAULT_MOONRAKER_PORT,
	};

	// let url = format!("ws://{hostname}:{port}/websocket");
	// let mut connection = FastMoonConn::new(url, None, None, false).await;
	// let mut conn = FastMoonConn::new(hostname, port, None, None, false).await?;
	// let mut connection = MoonrakerClient::new_simple(hostname, Some(port), false).await?;
	// let mut connection = MoonrakerClient::new(hostname, Some(port)).await?;
	let mut connection = MoonrakerClient::connect(hostname, Some(port))
		.await
		.unwrap();
	for _ in 0..10 {
		let msg = MoonRequest::new(MoonMethod::PrinterInfo, None);
		// let res = connection.send_listen(msg).await?;
		let res = connection.send_with_response(msg).await.unwrap();
		println!("Res: {res:?}");
	}

	// let conn = connect(hostname, port).await?;


	Ok(())
}

