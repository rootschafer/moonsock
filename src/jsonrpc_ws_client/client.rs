use std::{
	collections::HashMap,
	sync::{
		atomic::{AtomicUsize, Ordering},
		Arc,
	},
};

use serde::{Deserialize, Serialize};
// use url::Url;
use tokio::{
	sync::{
		mpsc::{self, error::SendError},
		oneshot, Mutex,
	},
	time::{Duration, Instant},
};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use tungstenite::{
	client::IntoClientRequest,
	// http::{Method, Request},
};

/// The default buffer size for the WebSocket writer.
pub const DEFAULT_WRITER_BUFFER_SIZE: usize = 1000;

/// The default buffer size for the WebSocket reader.
pub const DEFAULT_READER_BUFFER_SIZE: usize = 1000;

/// The default timeout for sending a message and listening for a response.
const DEFAULT_SEND_LISTEN_TIMEOUT: Duration = Duration::from_secs(60);
/// The default timeout for dirty pending requests.
const DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);


/// Represents a JSON-RPC version.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JsonRpcVersion {
	/// Version 2.0
	#[serde(rename = "2.0")]
	V2,
}

/// A JSON-RPC request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcRequest {
	/// The JSON-RPC version of the request.
	pub jsonrpc: JsonRpcVersion,
	/// The method to be called.
	pub method: serde_json::Value,
	/// The parameters to be passed to the method.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<serde_json::Value>,
	/// The ID of the request.
	id: u32,
}


impl JsonRpcRequest {
	/// Builds a new JSON-RPC request.
	///
	/// # Arguments
	///
	/// * `method` - The method to be called.
	/// * `params` - The parameters to be passed to the method.
	///
	/// # Returns
	///
	/// A `Result` containing the new request or an error if the request could not be built.
	pub fn build(
		method: impl Serialize,
		params: Option<impl Serialize>,
	) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
		let m = serde_json::to_value(&method).unwrap();
		match m {
			serde_json::Value::String(_) => Ok(Self {
				jsonrpc: JsonRpcVersion::V2,
				method: m,
				params: params.map(|p| serde_json::to_value(p).unwrap()),
				id: 0,
			}),
			_ => Err("Method must be type serializable into a String".into()),
		}
	}
}

/// A JSON-RPC response.
#[derive(Debug, Clone)]
pub enum JsonRpcResponse {
	/// A successful response.
	Result(JsonRpcSuccessResponse),
	/// An error response.
	ReturnedError(JsonRpcError),
}

/// A JSON-RPC error.
#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
	/// The error code.
	pub code: i32,
	/// The error message.
	pub message: String,
	/// Additional error data.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<serde_json::Value>,
}

impl std::fmt::Display for JsonRpcError {
	/// Formats the error for display.
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match &self.data {
			Some(data) => write!(f, "{}: message: {}: , data: {}", self.code, self.message, data),
			None => write!(f, "{}: {}", self.code, self.message),
		}
	}
}


/// A JSON-RPC error response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcErrorResponse {
	/// The JSON-RPC version of the response.
	pub jsonrpc: JsonRpcVersion,
	/// The error.
	pub error: JsonRpcError,
	/// The ID of the request.
	pub id: Option<u32>,
}

/// A JSON-RPC success response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcSuccessResponse {
	/// The JSON-RPC version of the response.
	pub jsonrpc: JsonRpcVersion,
	/// The result of the method call.
	pub result: serde_json::Value,
	/// The ID of the request.
	pub id: u32,
}

/// A JSON-RPC 2.0 notification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcNotification {
	/// The JSON-RPC version of the notification.
	pub jsonrpc: JsonRpcVersion,
	/// The method of the notification.
	pub method: String,
	/// The parameters of the notification.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<serde_json::Value>,
}

impl JsonRpcNotification {
	/// Builds a new JSON-RPC notification.
	///
	/// # Arguments
	///
	/// * `method` - The method of the notification.
	/// * `params` - The parameters of the notification.
	///
	/// # Returns
	///
	/// A `Result` containing the new notification or an error if the notification could not be built.
	pub fn build(
		method: impl Into<String>,
		params: Option<impl Serialize>,
	) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
		let method = method.into();
		let params = match params {
			Some(object) => match serde_json::to_value(object) {
				Ok(value) => Some(value),
				Err(e) => {
					return Err(format!("Could not serialize params into serde_json::Value: {e}").into());
				}
			},
			None => None,
		};
		// let params = params.map(|p| serde_json::to_value(p)?);
		Ok(Self { jsonrpc: JsonRpcVersion::V2, method, params })
	}
}


/// Represents the status of a pending request.
#[derive(Debug, PartialEq)]
enum PendingRequestStatus {
	/// The request is clean.
	Clean,
	/// The request is dirty.
	Dirty {
		timeout: Instant,
		error_message: JsonRpcError,
	},
}

type PendingRequests = Arc<Mutex<HashMap<u32, (oneshot::Sender<JsonRpcResponse>, PendingRequestStatus)>>>;

pub struct JsonRpcWsClient {
	/// The sender for sending messages to the WebSocket server.
	ws_writer: mpsc::Sender<JsonRpcRequest>,
	/// The receiver for receiving messages from the WebSocket server.
	ws_reader: mpsc::Receiver<JsonRpcResponse>,
	/// The sender for sending shutdown signals to the WebSocket server.
	shutdown_sender: mpsc::Sender<()>,
	/// A counter for generating unique IDs for messages.
	id_counter: AtomicUsize,
	/// A map of pending requests, where the key is the message ID and the value is a sender for sending the response.
	// pending_requests: Arc<Mutex<HashMap<u32, (oneshot::Sender<JsonRpcResponse>, PendingRequestStatus)>>>,
	pending_requests: PendingRequests,
	// notification_reader: mpsc::Receiver<serde_json::Value>,
	notification_reader: mpsc::Receiver<JsonRpcNotification>,
	// /// The timeout for sending a message and listening for a response.
	// send_listen_timeout: Duration,
	// /// The timeout for dirty pending requests.
	// dirty_pending_response_timeout: Duration,
}

impl Drop for JsonRpcWsClient {
	/// Sends a shutdown signal to the WebSocket server when the client is dropped.
	fn drop(&mut self) {
		let shutdown_sender = self.shutdown_sender.clone();
		tokio::spawn(async move {
			shutdown_sender.send(()).await.ok();
		});
	}
}

impl JsonRpcWsClient {
	/// Connects to a JSON-RPC WebSocket server.
	///
	/// # Arguments
	///
	/// * `url` - The URL of the WebSocket server.
	/// * `writer_buffer_size` - The buffer size for the writer. Defaults to `DEFAULT_WRITER_BUFFER_SIZE`.
	/// * `reader_buffer_size` - The buffer size for the reader. Defaults to `DEFAULT_READER_BUFFER_SIZE`.
	///
	/// # Returns
	///
	/// A `Result` containing the new client or an error if the client could not be created.
	pub async fn connect(
		url: String,
		writer_buffer_size: Option<usize>,
		reader_buffer_size: Option<usize>,
	) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
		// pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>) -> Result<Self, Box<dyn std::error::Error>> {
		let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
		let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);

		let (ws_writer_sender, mut ws_writer_receiver) = tokio::sync::mpsc::channel(writer_buffer_size);
		let (notification_sender, notification_reader) = tokio::sync::mpsc::channel(1000);
		let (shutdown_sender, mut shutdown_receiver) = tokio::sync::mpsc::channel(10);

		let request = url.into_client_request().expect("Invalid URL");
		let (ws_stream, _) = connect_async(request).await?;

		tracing::debug!("WebSocket handshake has been successfully completed");

		let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
		let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

		tokio::spawn(async move {
			loop {
				tokio::select! {
					Some(msg) = ws_writer_receiver.recv() => {
						tracing::debug!("Sending message: {:?}", serde_json::to_string_pretty(&msg).unwrap());
						match serde_json::to_vec(&msg) {
							Ok(mut vec) => {
								vec.truncate(vec.len());

								match Pin::new(&mut moon_socket_sink).send(Message::binary(vec)).await {
									Ok(_) => {},
									Err(e) => eprintln!("Error sending to moon_socket_sink: {e}"),
								}
							},
							Err(e) => eprintln!("Error serializing request: {e}"),
						}
					},
					_ = websocket_shutdown_receiver.recv() => {
						break;
					},
				}
			}
		});

		let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);

		let pending_requests = Arc::new(Mutex::new(HashMap::<
			u32,
			(oneshot::Sender<JsonRpcResponse>, PendingRequestStatus),
		>::new()));
		let pending_requests_clone = Arc::clone(&pending_requests);

		tokio::spawn(async move {
			let mut dirty_pending_requests: HashMap<Instant, Vec<u32>> = HashMap::new();
			let mut has_dirty_requests = false;

			loop {
				tokio::select! {
					Some(message) = moon_socket_stream.next() => {
						match message {
							Ok(msg) => {
								if msg.is_empty() {
									continue;
								}
								let message_txt = match msg.into_text() {
									Ok(txt) => txt,
									Err(e) => {
										tracing::error!("Error converting message to text: {}", e);
										continue;
									}
								};
								let maybe_notification: Result<JsonRpcNotification, serde_json::Error> = serde_json::from_str(&message_txt);
								match maybe_notification {
									Ok(notification) => {
										notification_sender.send(notification).await.ok();
									},
									Err(_) => {
										let maybe_success_response: Result<JsonRpcSuccessResponse, serde_json::Error> = serde_json::from_str(&message_txt);
										match maybe_success_response {
											Ok(message) => {
												tracing::debug!("Response: {}", message_txt);
												let id = message.id;
												if let Some((tx, _)) = pending_requests_clone.lock().await.remove(&id) {
													tx.send(JsonRpcResponse::Result(message)).ok();
												} else {
													ws_reader_sender.send(JsonRpcResponse::Result(message)).await.ok();
												}
											},
											Err(_) => {
												let parsed: Result<JsonRpcErrorResponse, serde_json::Error> = serde_json::from_str(&message_txt);
												match parsed {
													Ok(message) => {
														tracing::error!("Error: {}", message_txt);
														match message.id {
															Some(id) => {
																if let Some((tx, _)) = pending_requests_clone.lock().await.remove(&id) {
																	tx.send(JsonRpcResponse::ReturnedError(message.error.clone())).ok();
																} else {
																	ws_reader_sender.send(JsonRpcResponse::ReturnedError(message.error.clone())).await.ok();
																}
															},
															None => {
																// Handle error with null id
																let mut pending_requests_lock = pending_requests_clone.lock().await;
																if pending_requests_lock.len() == 1 {
																	// If there is only one pending request, send the error through the oneshot channel
																	let (_, (tx, _)) = pending_requests_lock.drain().next().unwrap();
																	tx.send(JsonRpcResponse::ReturnedError(message.error.clone())).ok();
																} else {
																	let now = Instant::now();
																	for (id, (_, status)) in pending_requests_lock.iter_mut() {
																		if *status == PendingRequestStatus::Clean {
																			*status = PendingRequestStatus::Dirty {
																				timeout: now + DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT,
																				error_message: message.error.clone(),
																			};
																			// dirty_pending_requests.entry(now + DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT).or_insert(Vec::new()).push(*id);
																			dirty_pending_requests.entry(now + DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT).or_default().push(*id);
																		}
																	}
																	has_dirty_requests = true;
																}
															},
														}
													},
													Err(_) => {
														tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
														tracing::error!("Message Length: {}", message_txt.len());
														tracing::error!("{}", message_txt);
														tracing::error!("--------------------------------------------------------------------------");
													},
												}
											},
										}
									},
								}
								// let raw_value: serde_json::Value = serde_json::from_str(&message_txt).unwrap();
								// if let Some(_id) = raw_value.get("id") {
								//     // Handle response
								//     let parsed: Result<JsonRpcSuccessResponse, serde_json::Error> = serde_json::from_str(&message_txt);
								//     match parsed {
								//         Ok(message) => {
								//             tracing::debug!("Response: {}", message_txt);
								//             let id = message.id;
								//             if let Some((tx, _)) = pending_requests_clone.lock().await.remove(&id) {
								//                 tx.send(JsonRpcResponse::Result(message)).ok();
								//             } else {
								//                 ws_reader_sender.send(JsonRpcResponse::Result(message)).await.ok();
								//             }
								//         },
								//         Err(_) => {
								//             let parsed: Result<JsonRpcErrorResponse, serde_json::Error> = serde_json::from_str(&message_txt);
								//             match parsed {
								//                 Ok(message) => {
								//                     tracing::error!("Error: {}", message_txt);
								//                     match message.id {
								//                         Some(id) => {
								//                             if let Some((tx, _)) = pending_requests_clone.lock().await.remove(&id) {
								//                                 tx.send(JsonRpcResponse::ReturnedError(message.error.clone())).ok();
								//                             } else {
								//                                 ws_reader_sender.send(JsonRpcResponse::ReturnedError(message.error.clone())).await.ok();
								//                             }
								//                         },
								//                         None => {
								//                             // Handle error with null id
								//                             let mut pending_requests_lock = pending_requests_clone.lock().await;
								//                             if pending_requests_lock.len() == 1 {
								//                                 // If there is only one pending request, send the error through the oneshot channel
								//                                 let (_, (tx, _)) = pending_requests_lock.drain().next().unwrap();
								//                                 tx.send(JsonRpcResponse::ReturnedError(message.error.clone())).ok();
								//                             } else {
								//                                 let now = Instant::now();
								//                                 for (id, (_, status)) in pending_requests_lock.iter_mut() {
								//                                     if *status == PendingRequestStatus::Clean {
								//                                         *status = PendingRequestStatus::Dirty {
								//                                             timeout: now + DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT,
								//                                             error_message: message.error.clone(),
								//                                         };
								//                                         dirty_pending_requests.entry(now + DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT).or_insert(Vec::new()).push(*id);
								//                                     }
								//                                 }
								//                                 has_dirty_requests = true;
								//                             }
								//                         },
								//                     }
								//                 },
								//                 Err(_) => {
								//                     tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
								//                     tracing::error!("Message Length: {}", message_txt.len());
								//                     tracing::error!("{}", message_txt);
								//                     tracing::error!("--------------------------------------------------------------------------");
								//                 },
								//             }
								//         },
								//     }
								// } else {
								//     // Handle notification
								//     tracing::trace!("Received Notification: \n{}", message_txt);
								//     // notification_sender.send(raw_value).await.ok();

								//     let notification: JsonRpcNotification = serde_json::from_value(raw_value)?;
								//     notification_sender.send(notification).await.ok();
								// }
							},
							Err(e) => tracing::error!("Error message from moonraker socket: {}", e.to_string()),
						}
					},
					_ = shutdown_receiver.recv() => {
						match websocket_shutdown_sender.send(()).await {
							Ok(_) => {},
							Err(e) => {
								tracing::error!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
							},
						}
						break;
					},
					_ = tokio::time::sleep(Duration::from_millis(100)) => {
						// Check for dirty pending request timeouts
						if has_dirty_requests {
							let now = Instant::now();
							let mut timed_out_requests = Vec::new();
							for (timeout, ids) in dirty_pending_requests.iter() {
								if *timeout <= now {
									timed_out_requests.push((*timeout, ids.clone()));
								}
							}
							for (timeout, ids) in timed_out_requests {
								let mut valid_ids = Vec::new();
								for id in ids {
									if pending_requests_clone.lock().await.contains_key(&id) {
										valid_ids.push(id);
									}
								}
								let count = valid_ids.len();
								for id in valid_ids {
									// if let Some((tx, status)) = pending_requests_clone.lock().await.remove(&id) {
									// 	if let PendingRequestStatus::Dirty { error_message, .. } = status {
									// 		let mut error_message = error_message.clone();
									// 		if count > 1 {
									// 			error_message.message = format!("Error: This error may not be associated with this request and may have originated from another. One of your requests had an id that the websocket server could not parse. There is probably something wrong with thewebsocket server is parsing ids.\n{}", error_message.message);
									// 		}
									// 		tx.send(JsonRpcResponse::ReturnedError(error_message)).ok();
									// 	}
									// }

									// if let Some((tx, status)) = pending_requests_clone.lock().await.remove(&id) {
									if let Some((tx, PendingRequestStatus::Dirty { error_message, .. })) = pending_requests_clone.lock().await.remove(&id) {
										// if let PendingRequestStatus::Dirty { error_message, .. } = status {
											let mut error_message = error_message.clone();
											if count > 1 {
												error_message.message = format!("Error: This error may not be associated with this request and may have originated from another. One of your requests had an id that the websocket server could not parse. There is probably something wrong with thewebsocket server is parsing ids.\n{}", error_message.message);
											}
											tx.send(JsonRpcResponse::ReturnedError(error_message)).ok();
										// }
									}
								}
								dirty_pending_requests.remove(&timeout);
							}
							if dirty_pending_requests.is_empty() {
								has_dirty_requests = false;
							}
						}
					}
				}
			}
		});

		Ok(JsonRpcWsClient {
			ws_writer: ws_writer_sender,
			ws_reader: ws_reader_receiver,
			shutdown_sender,
			id_counter: AtomicUsize::new(1),
			pending_requests,
			notification_reader,
		})
	}

	/// Receives a message from the WebSocket server.
	///
	/// # Returns
	///
	/// A `JsonRpcResponse` if a message is available, or `None` if the channel is closed.
	pub async fn recv(&mut self) -> Option<JsonRpcResponse> {
		// match self.ws_reader.recv().await {
		// 	Some(message) => Some(message),
		// 	None => None,
		// }

		self.ws_reader.recv().await
	}

	/// Listens for a notification from the WebSocket server.
	///
	/// # Returns
	///
	/// A `serde_json::Value` if a notification is available, or `None` if the channel is closed.
	pub async fn listen_for_notification(&mut self) -> Option<JsonRpcNotification> {
		// match self.notification_reader.recv().await {
		// 	Some(notification) => Some(notification),
		// 	None => None,
		// }

		self.notification_reader.recv().await
	}

	// pub async fn listen_for_notification(&mut self) -> Option<serde_json::Value> {
	//     match self.notification_reader.recv().await {
	//         Some(message) => Some(message),
	//         None => None,
	//     }
	// }

	/// Sends a message to the WebSocket server without waiting for a response.
	///
	/// # Arguments
	///
	/// * `message` - The message to send.
	///
	/// # Returns
	///
	/// A `Result` containing the ID of the sent message, or an error if the send operation failed.
	///
	/// # Note
	///
	/// If you expect a response from the server for your message, it is recommended to use `send_with_response` instead of this method. `send_with_response` will wait for a response from the server with a matching ID and return the response. Using `send_no_response` and then looping with `recv` to wait for a response with a matching ID is less efficient and may lead to bugs.
	pub async fn send_no_response(&mut self, mut message: JsonRpcRequest) -> Result<u32, SendError<JsonRpcRequest>> {
		let id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
		message.id = id;
		self.ws_writer.send(message).await?;
		Ok(id)
	}

	/// Sends a message to the WebSocket server and waits for a response.
	///
	/// # Arguments
	///
	/// * `message` - The message to send.
	///
	/// # Returns
	///
	/// A `Result` containing the response from the server, or an error if the response could not be received.
	///
	/// # Under-the-hood
	///
	/// This method adds a unique ID to the request and then waits to receive a response from the server with an ID that matches the unique ID. This ensures that the response is correctly matched with the request. Using `send_no_response` and then looping with `recv` to wait for a response with a matching ID is less efficient and may lead to bugs.
	pub async fn send_with_response(
		&mut self,
		mut message: JsonRpcRequest,
	) -> Result<JsonRpcResponse, Box<dyn std::error::Error + Send + Sync>> {
		// pub async fn send_listen(&mut self, mut message: JsonRpcRequest) -> Result<JsonRpcResponse, Box<dyn std::error::Error>> {
		let id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
		message.id = id;

		let (tx, rx) = oneshot::channel();
		self.pending_requests
			.lock()
			.await
			.insert(id, (tx, PendingRequestStatus::Clean));

		// self.send_no_response(message).await?; // Send the message
		self.ws_writer.send(message).await?; // Send the message

		match tokio::time::timeout(DEFAULT_SEND_LISTEN_TIMEOUT, rx).await {
			// Example timeout of 5 seconds
			Ok(Ok(response)) => Ok(response),
			Ok(Err(_)) => Err("Channel closed unexpectedly".to_string().into()),
			Err(_) => Err("Timeout waiting for response".to_string().into()),
		}
	}
}
