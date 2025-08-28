// use moonsock::moonraker_client_new::MoonrakerClient;
use std::env;

use moonsock::MoonrakerClient;

const DEFAULT_MOONRAKER_PORT: u16 = 7125;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
	let hostname = env::var("MOONRAKER_HOSTNAME").expect("Please add the `MOONRAKER_HOSTNAME` environment variable");
	let port = match env::var("MOONRAKER_PORT") {
		Ok(port_string) => match port_string.parse::<u16>() {
			Ok(port) => port,
			Err(_e) => DEFAULT_MOONRAKER_PORT,
		},
		Err(_e) => DEFAULT_MOONRAKER_PORT,
	};
	let mut connection = MoonrakerClient::connect(hostname, Some(port))
		.await
		.unwrap();

	match connection.get_printer_info().await {
		Ok(printer_info) => {
			let state = printer_info.state;
			println!("Printer State: {state:?}");
		}
		Err(e) => {
			eprintln!("Error getting printer info: {e}");
		}
	}
	Ok(())
}
