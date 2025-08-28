// use moonsock::connection::PrinterSafetyStatus;
// use moonsock::PrinterSafetyStatus;
// use moonsock::moonraker_client_new::PrinterSafetyStatus;
// use moonsock::FastMoonConn;
// use moonsock::MoonConnection;
// use moonsock::MoonrakerClient;
// use moonsock::moonraker_client_new::MoonrakerClient;
use std::env;

use moonsock::{MoonrakerClient, PrinterSafetyStatus};
// use tracing::subscriber;
// use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

// use tracing_subscriber::FmtSubscriber;

const DEFAULT_MOONRAKER_PORT: u16 = 7125;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
	tracing_subscriber::registry()
		.with(fmt::layer())
		.with(EnvFilter::from_default_env())
		.init();

	let hostname = env::var("MOONRAKER_HOSTNAME").expect("Please add the `MOONRAKER_HOSTNAME` environment variable");
	let port = match env::var("MOONRAKER_PORT") {
		Ok(port_string) => match port_string.parse::<u16>() {
			Ok(port) => port,
			Err(_e) => DEFAULT_MOONRAKER_PORT,
		},
		Err(_e) => DEFAULT_MOONRAKER_PORT,
	};

	// let url = format!("ws://{hostname}:{port}/websocket");
	// let mut connection = FastMoonConn::new(url.to_string(), 1000, 1000, false).await;
	// let mut connection = FastMoonConn::new(hostname, port, None, None, true).await?;
	// let mut connection = MoonConnection::new_simple(hostname, Some(port), false).await?;
	// let mut connection = MoonrakerClient::new_simple(hostname, Some(port), false).await?;
	let mut connection = MoonrakerClient::connect(hostname, Some(port))
		.await
		.unwrap();
	let username = env::var("MOONRAKER_USERNAME").expect("Please add the `MOONRAKER_USERNAME` environment variable");
	let password = env::var("MOONRAKER_PASSWORD").expect("Please add the `MOONRAKER_PASSWORD` environment variable");
	// connection.create_user(&username, &password).await?;
	connection.authenticate(username, password).await.unwrap();
	println!("Connected to moonraker");
	match connection.ensure_ready().await {
		PrinterSafetyStatus::Ready => println!("Printer is ready!"),
		PrinterSafetyStatus::KlipperError(e) => eprintln!("Error: {e}"),
		PrinterSafetyStatus::OtherError(e) => eprintln!("Error: {e}"),
		PrinterSafetyStatus::Maybe3DPrintInsidePrinter(state) => {
			eprintln!("Error: There could be a print inside the printer! Printer State: {state:?}")
		}
		PrinterSafetyStatus::Shutdown => eprintln!("Error: The printer is shutting down!"),
		PrinterSafetyStatus::TimeoutReached => eprintln!("Error: The printer timed out!"),
		PrinterSafetyStatus::TooManyRestarts => eprintln!("Error: The printer restarted too many times!"),
	}
	let is_homed = match connection.is_homed().await {
		Ok(homed) => homed,
		Err(e) => {
			println!("Error getting printer info: {e}");
			Err(e)?
		}
	};

	if is_homed {
		println!("Printer is homed!");
	} else {
		println!("Printer is not homed");
	}
	Ok(())
}
