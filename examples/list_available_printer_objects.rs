// use moonsock::{
//     // FastMoonConn,
//     MoonrakerClient,
//     // MoonMSG,
//     MoonRequest, MoonResponse,
//     MoonMethod, response::MoonResultData
// };
// use std::env;

// const DEFAULT_MOONRAKER_PORT: u16 = 7125;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//     let hostname = env::var("MOONRAKER_HOSTNAME").expect("Please add the `MOONRAKER_HOSTNAME` environment variable");
//     let port = match env::var("MOONRAKER_PORT") {
//         Ok(port_string) => {
//             match port_string.parse::<u16>() {
//                 Ok(port) => port,
//                 Err(_e) => DEFAULT_MOONRAKER_PORT,
//             }
//         },
//         Err(_e) => DEFAULT_MOONRAKER_PORT,
//     };

//     // let url = format!("ws://{hostname}:{port}/websocket");
//     // let mut connection = MoonConnection::new(url, 1000, 1000).await;
//     // let mut connection = FastMoonConn::new(hostname, port, None, None, false).await?;
//     // let mut connection = MoonrakerClient::new_simple(hostname, Some(port), false).await?;
//     let mut connection = MoonrakerClient::new(hostname, Some(port)).await?;
//     // let msg = MoonMSG::new(MoonMethod::PrinterObjectsList, None, Some(19876)); // Choose a message ID
//     let msg = MoonRequest::new(MoonMethod::PrinterObjectsList, None);
//     let response = connection.send_listen(msg).await?;

//     match response {
//         // MoonMSG::MoonResult { result, .. } => match result {
//         MoonResponse::MoonResult { result, .. } => match result {
//             MoonResultData::PrinterObjectsListResponse(data) => {
//                 println!("Available Printer Objects: {:?}", data.objects);
//             }
//             _ => println!("Unexpected response format"),
//         },
//         _ => println!("Unexpected response type"),
//     }

//     Ok(())
// }


use std::{env, error::Error};

use moonsock::{
	MoonMethod,
	MoonRequest,
	MoonResponse,
	MoonResultData,
	// moonraker_client_new::MoonrakerClient,
	MoonrakerClient,
};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const DEFAULT_MOONRAKER_PORT: u16 = 7125;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	tracing_subscriber::registry()
		.with(fmt::layer())
		.with(EnvFilter::from_default_env())
		.init();

	let hostname = env::var("MOONRAKER_HOSTNAME")?;
	let port = env::var("MOONRAKER_PORT")?
		.parse::<u16>()
		.unwrap_or(DEFAULT_MOONRAKER_PORT);

	let mut connection = MoonrakerClient::connect(hostname, Some(port))
		.await
		.unwrap();
	let username = env::var("MOONRAKER_USERNAME")?;
	let password = env::var("MOONRAKER_PASSWORD")?;
	connection.authenticate(username, password).await.unwrap();
	println!("Connected to moonraker");

	let msg = MoonRequest::new(MoonMethod::PrinterObjectsList, None);
	let response = connection.send_with_response(msg, None).await.unwrap();

	match response {
		MoonResponse::MoonResult { result, .. } => match result {
			MoonResultData::PrinterObjectsListResponse(data) => {
				println!("Available Printer Objects: {:?}", data.objects);
			}
			_ => println!("Unexpected response format"),
		},
		_ => println!("Unexpected response type"),
	}

	Ok(())
}
