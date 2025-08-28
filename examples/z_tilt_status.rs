// use moonsock::FastMoonConn;
// use moonsock::MoonrakerClient;
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

	// let url = format!("ws://{hostname}:{port}/websocket");
	// let mut connection = MoonConnection::new(url, 1000, 1000).await;
	// let mut connection = FastMoonConn::new(hostname, port, None, None, false).await?;
	// let mut connection = MoonrakerClient::new_simple(hostname, Some(port), false).await?;
	// let mut connection = MoonrakerClient::new(hostname, Some(port)).await?;
	let mut connection = MoonrakerClient::connect(hostname, Some(port))
		.await
		.unwrap();

	let is_z_tilt_appled = connection.is_z_tilt_applied().await?;
	if is_z_tilt_appled {
		println!("Z-Tilt is applied!");
	} else {
		println!("Z-Tilt is not applied");
	}

	// let msg = MoonMSG::new(
	//     MoonMethod::PrinterObjectsQuery,
	//     Some(MoonParam::PrinterObjectsQuery {
	//         objects: PrinterObject::ZTilt(None),
	//     }),
	//     Some(19876), // Choose a message ID
	// );

	// match connection.send_listen(msg).await? {
	//     MoonMSG::MoonResult { result, .. } => match result {
	//         MoonResultData::QueryPrinterObjectsResponse(data) => {
	//             if let Some(z_tilt) = data.status.z_tilt {
	//                 if z_tilt.applied {
	//                     println!("Z-Tilt is applied!");
	//                 } else {
	//                     println!("Z-Tilt is not applied");
	//                 }
	//             } else {
	//                 println!("Z-Tilt object not found in printer response.");
	//             }
	//         }
	//         _ => println!("Unexpected response format"),
	//     },
	//     _ => println!("Unexpected response type"),
	// }

	Ok(())
}
