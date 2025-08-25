use std::{
	error::Error as StdError,
	time::{Duration, Instant},
};

// use serde::Serialize;
use tokio::{
	io::{stdin, stdout, AsyncWriteExt},
	sync::mpsc::{self, error::SendError},
	time::sleep,
};
use tokio_util::codec::{FramedRead, LinesCodec};
use spinoff::{spinners, Color, Spinner};
use futures_util::StreamExt;

use crate::{
	response::{PrinterState, ServerInfo},
	MoonNotification,
};
use crate::{
	// jsonrpc_ws_client::JsonRpcWsClient,
	jsonrpc_ws_client::{
		JsonRpcRequest,
		// JsonRpcResponse,
		JsonRpcWsClient,
	},
	// connection::PrinterSafetyStatus,
	response::{MoonResultData, PrinterInfoResponse},
	MoonErrorContent,
	MoonMethod,
	MoonParam,
	MoonRequest,
	MoonResponse,
	PrinterObject,
};



/// The default port used by Moonraker.
const DEFAULT_MOONRAKER_PORT: u16 = 7125;

/// The timeout in seconds for waiting for the printer to be ready.
const PRINTER_READY_TIMEOUT: u64 = 240;

/// The maximum number of restart attempts.
const MAX_RESTARTS: u8 = 3;


#[derive(Debug)]
pub enum PrinterSafetyStatus {
	Ready,
	Maybe3DPrintInsidePrinter(PrinterState),
	KlipperError(String),
	Shutdown,
	TimeoutReached,
	TooManyRestarts,
	OtherError(Box<dyn std::error::Error>),
}

/// An error that can occur when sending a message to Moonraker.
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum MoonSendError<T> {
	/// An error occurred while sending a message.
	#[error("Error sending message: {0}")]
	SendError(#[from] mpsc::error::SendError<T>),
	/// A Moonraker error occurred.
	#[error("Moonraker error: {0}")]
	MoonError(MoonErrorContent),
	/// A general error occurred.
	#[error("Error: {0}")]
	String(String),
}

/// Converts a `Box<dyn StdError>` to a `MoonSendError`.
impl<T> From<Box<dyn StdError>> for MoonSendError<T> {
	fn from(err: Box<dyn StdError>) -> Self {
		MoonSendError::String(err.to_string())
	}
}

/// A client for communicating with Moonraker.
pub struct MoonrakerClient {
	/// The underlying JSON-RPC WebSocket client.
	// connection: JsonRpcWsClient<MoonRequest, MoonResponse>,
	connection: JsonRpcWsClient,
}

impl MoonrakerClient {
	/// Creates a new `MoonrakerClient` with the given hostname and port.
	pub async fn connect(hostname: String, port: Option<u16>) -> Result<MoonrakerClient, Box<dyn std::error::Error + Send + Sync>> {
		let port = port.unwrap_or(DEFAULT_MOONRAKER_PORT);
		let url = format!("ws://{hostname}:{port}/websocket");
		Self::connect_with_buffer_sizes(url, None, None).await
	}

	/// Creates a new `MoonrakerClient` with the given URL and buffer sizes.
	pub async fn connect_with_buffer_sizes(
		url: String,
		writer_buffer_size: Option<usize>,
		reader_buffer_size: Option<usize>,
	) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
		let connection = JsonRpcWsClient::connect(url, writer_buffer_size, reader_buffer_size).await?;
		Ok(MoonrakerClient { connection })
	}

	/// Sends a message to Moonraker without waiting for a response.
	pub async fn send(&mut self, message: MoonRequest) -> Result<(), SendError<JsonRpcRequest>> {
		self.connection.send_no_response(message.into()).await?;
		Ok(())
	}

	/// Sends a message to Moonraker and waits for a response.
	pub async fn send_with_response(
		&mut self,
		message: MoonRequest,
	) -> Result<MoonResponse, Box<dyn std::error::Error + Send + Sync>> {
		let response = self.connection.send_with_response(message.into()).await?;
		Ok(response.into())
	}

	/// Sends a message to Moonraker and waits for an OK response.
	pub async fn send_wait_for_ok(&mut self, message: MoonRequest) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let res = match self.connection.send_with_response(message.into()).await {
			Ok(res) => res.into(),
			Err(e) => {
				tracing::error!("Error sending message: {}", e);
				return Err(e);
			}
		};
		match res {
			MoonResponse::MoonResult { result: MoonResultData::Ok(..), .. } => Ok(()),
			// Should never be possible to get the `RES::MoonError` variant as long as the logic of `send_with_response` never changes, but
			// for correctness reasons we should still check for it.
			MoonResponse::MoonError { error, .. } => {
				tracing::error!("Error: {:?}", error);
				Err(error.into())
			}
			_ => {
				tracing::error!("Expected an Ok response got: {:?}", res);
				Err(format!("Expected an Ok response got: {res:?}").into())
			}
		}
	}

	/// Listens for a JSON-RPC notification from Moonraker.
	/// Returns None if the underlying channel is closed.
	// pub async fn listen_for_notification(&mut self) -> Option<crate::jsonrpc_ws_client::JsonRpcNotification> {
	// pub async fn listen_for_notification(&mut self) -> Option<MoonNotification> {
	pub async fn listen_for_notification(&mut self) -> Option<Result<MoonNotification, Box<dyn std::error::Error + Send + Sync>>> {
		match self.connection.listen_for_notification().await {
			Some(notification) => Some(notification.try_into()),
			None => None,
		}
	}

	pub async fn create_user(
		&mut self,
		username: impl Into<String>,
		password: impl Into<String>,
	) -> Result<MoonResponse, Box<dyn std::error::Error + Send + Sync>> {
		let message = MoonRequest::new(
			MoonMethod::AccessPostUser,
			Some(MoonParam::AccessPostUserParams {
				username: username.into(),
				password: password.into(),
			}),
		);
		self.send_with_response(message).await
	}

	pub async fn authenticate(
		&mut self,
		username: String,
		password: String,
	) -> Result<MoonResponse, Box<dyn std::error::Error + Send + Sync>> {
		let params = MoonParam::AccessLoginParams {
			username,
			password,
			source: "moonraker".to_string(),
		};
		let message = MoonRequest::new(MoonMethod::AccessLogin, Some(params));
		self.send_with_response(message).await
	}

	/// Ensures that the printer is ready.
	pub async fn ensure_ready(&mut self) -> PrinterSafetyStatus {
		let mut sp = Spinner::new(spinners::Dots, "Loading...", Color::Blue);

		let start_time = Instant::now();
		let mut restart_count = 0;

		loop {
			if start_time.elapsed() > Duration::from_secs(PRINTER_READY_TIMEOUT) {
				sp.stop();
				return PrinterSafetyStatus::TimeoutReached;
			}

			let printer_state = match self.get_server_info().await {
				Ok(info) => info.klippy_state,
				Err(err) => {
					sp.stop();
					tracing::error!("Error getting server info: {}", err);
					return PrinterSafetyStatus::OtherError(err);
				}
			};

			match printer_state {
				PrinterState::Ready | PrinterState::Standby => {
					sp.stop_with_message("Printer is ready!");
					return PrinterSafetyStatus::Ready;
				}
				PrinterState::Startup => {
					sp.update(spinners::Dots9, "Printer is starting up..", None);
					sleep(Duration::from_millis(500)).await; // Shorter wait with spinner
					continue;
				}
				PrinterState::Paused | PrinterState::Printing | PrinterState::Complete | PrinterState::Cancelled => {
					sp.stop_with_message(
						format!("Printer is in {printer_state:?} state. Please clear the print bed.").as_str(),
					);
					return PrinterSafetyStatus::Maybe3DPrintInsidePrinter(printer_state);
				}
				PrinterState::Error => {
					let error_message = match self.get_server_info().await {
						Ok(info) => info.warnings.join(", "),
						Err(_) => "Unknown error".to_string(),
					};
					if restart_count >= MAX_RESTARTS {
						sp.stop_and_persist("❌", "Too many firmware restarts");
						return PrinterSafetyStatus::TooManyRestarts;
					}

					sp.stop();
					tracing::error!("Printer in Error state: {}", error_message);
					if self.prompt_for_restart().await.unwrap() {
						sp = Spinner::new(spinners::Dots9, "Running firmware restart...", Color::Blue);
						self.firmware_restart().await.ok(); // Ignore errors during restart
						restart_count += 1;
					} else {
						return PrinterSafetyStatus::KlipperError(error_message);
					}
				}
				PrinterState::Shutdown => {
					if restart_count >= MAX_RESTARTS {
						sp.stop_and_persist("❌", "Too many firmware restarts");
						return PrinterSafetyStatus::TooManyRestarts;
					}
					sp.stop();
					if self.prompt_for_restart().await.unwrap() {
						sp = Spinner::new(spinners::Dots9, "Running firmware restart...", Color::Blue);
						self.firmware_restart().await.ok(); // Ignore errors during restart
						restart_count += 1;
					} else {
						return PrinterSafetyStatus::Shutdown;
					}
				}
				PrinterState::Disconnected => {
					sp.update(spinners::Dots9, "Printer is disconnected", None);
				}
			}
		}
	}

	/// Prompts the user to restart the firmware.
	async fn prompt_for_restart(&self) -> Result<bool, Box<dyn std::error::Error>> {
		let mut stdout = stdout();
		stdout
			.write_all(b"Do you want to attempt a firmware restart? (y/n): ")
			.await?;
		stdout.flush().await?;

		let stdin = stdin();
		let mut reader = FramedRead::new(stdin, LinesCodec::new());
		let input = reader.next().await.transpose()?.unwrap();
		Ok(input.trim().to_lowercase() == "y")
	}

	/// Restarts the firmware.
	async fn firmware_restart(&mut self) -> Result<(), Box<dyn StdError>> {
		let message = MoonRequest::new(MoonMethod::PrinterFirmwareRestart, None);
		match self.send(message).await {
			Ok(_) => Ok(()),
			Err(e) => {
				tracing::error!("Error sending firmware restart message: {}", e);
				Err(e.into())
			}
		}
	}

	/// Checks if the printer is ready.
	pub async fn is_printer_ready(&mut self) -> Result<bool, Box<dyn StdError>> {
		let server_info = match self.get_server_info().await {
			Ok(info) => info,
			Err(e) => {
				tracing::error!("Error getting server info: {}", e);
				return Err(e);
			}
		};
		Ok(server_info.klippy_state == PrinterState::Ready)
	}

	/// Gets the server information.
	pub async fn get_server_info(&mut self) -> Result<ServerInfo, Box<dyn std::error::Error>> {
		let message = MoonRequest::new(MoonMethod::ServerInfo, None);
		let res = match self.send_with_response(message).await {
			Ok(res) => res,
			Err(e) => {
				tracing::error!("Error sending message: {}", e);
				return Err(e);
			}
		};
		match res {
			MoonResponse::MoonResult { result, .. } => match result {
				MoonResultData::ServerInfo(server_info) => Ok(server_info),
				_ => {
					tracing::error!("Unexpected response: {:?}", result);
					Err(format!("Unexpected response: {result:?}").into())
				}
			},
			_ => {
				tracing::error!("Unexpected response: {:?}", res);
				Err(format!("Unexpected response: {res:?}").into())
			}
		}
	}

	/// Gets the printer information.
	pub async fn get_printer_info(&mut self) -> Result<PrinterInfoResponse, Box<dyn std::error::Error>> {
		let message = MoonRequest::new(MoonMethod::PrinterInfo, None);
		let res = match self.send_with_response(message).await {
			Ok(res) => res,
			Err(e) => {
				tracing::error!("Error sending message: {}", e);
				return Err(e);
			}
		};
		match res {
			MoonResponse::MoonResult { result, .. } => match result {
				MoonResultData::Ok(_) => {
					tracing::error!("Received an ok() response from the server, but was expecting ");
					Err("Received an ok() response from the server, but was expecting ".into())
				}
				MoonResultData::PrinterInfoResponse(printer_info) => Ok(printer_info),
				_ => {
					tracing::error!("Error in `MoonrakerClient::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.");
					Err("Error in `MoonrakerClient::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.".into())
				}
			},
			MoonResponse::MoonError { error, .. } => Err(error.into()),
			_ => {
				tracing::error!("Error in `MoonrakerClient::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.");
				Err("Error in `MoonrakerClient::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.".into())
			}
		}
	}

	/// Gets the homed axes.
	pub async fn get_homed_axes(&mut self) -> Result<String, Box<dyn std::error::Error>> {
		let param = MoonParam::PrinterObjectsQuery {
			objects: PrinterObject::Toolhead(Some(vec!["homed_axes".to_string()])),
		};
		let msg = MoonRequest::new(MoonMethod::PrinterObjectsQuery, Some(param));

		match self.send_with_response(msg).await {
			Ok(res) => match res {
				MoonResponse::MoonResult { result, .. } => match result {
					MoonResultData::PrinterObjectsQueryResponse(res) => match res.status.toolhead {
						Some(toolhead) => match toolhead.homed_axes {
							Some(homed_axes) => Ok(homed_axes),
							None => {
								tracing::error!("Error: Could not find `homed_axes` in response from printer");
								Err("Error: Could not find `homed_axes` in response from printer".into())
							}
						},
						None => {
							tracing::error!("Error: Could not find the `toolhead` field in response from printer");
							Err("Error: Could not find the `toolhead` field in response from printer".into())
						}
					},
					_ => {
						tracing::error!("Error: Printer did not return expected response");
						Err("Error: Printer did not return expected response".into())
					}
				},
				_ => {
					tracing::error!("Error: Printer did not return expected response");
					Err("Error: Printer did not return expected response".into())
				}
			},
			Err(e) => {
				tracing::error!("Error sending message: {}", e);
				Err(e)
			}
		}
	}

	/// Checks if the printer is homed.
	pub async fn is_homed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
		let homed_axes = match self.get_homed_axes().await {
			Ok(homed_axes) => homed_axes,
			Err(e) => {
				tracing::error!("Error getting printer info: {}", e);
				return Err(e);
			}
		};

		// Assuming 'XYZ' or similar indicates all required axes are homed
		Ok(homed_axes.to_lowercase().contains("xyz"))
	}

	/// Checks if the Z tilt is applied.
	pub async fn is_z_tilt_applied(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
		let param = MoonParam::PrinterObjectsQuery { objects: PrinterObject::ZTilt(None) };
		let msg = MoonRequest::new(MoonMethod::PrinterObjectsQuery, Some(param));

		match self.send_with_response(msg).await {
			Ok(res) => match res {
				MoonResponse::MoonResult { result, .. } => match result {
					MoonResultData::PrinterObjectsQueryResponse(res) => match res.status.z_tilt {
						Some(z_tilt) => Ok(z_tilt.applied),
						None => {
							tracing::error!("Error: 'z_tilt' object not found in response");
							Err("Error: 'z_tilt' object not found in response".into())
						}
					},
					_ => {
						tracing::error!("Error: Unexpected response format from Moonraker");
						Err("Error: Unexpected response format from Moonraker".into())
					}
				},
				_ => {
					tracing::error!("Error: Unexpected response type from Moonraker");
					Err("Error: Unexpected response type from Moonraker".into())
				}
			},
			Err(e) => {
				tracing::error!("Error sending message: {}", e);
				Err(e)
			}
		}
	}
}
