// use moonsock::FastMoonConn;

// #[tokio::test]
// async fn printer_info_parsing() {
//     // let url = "ws://scanhead.local:7125/websocket";
//     let hostname = std::env::var("MOONRAKER_HOSTNAME").unwrap_or("localhost".to_string());
//     let port = std::env::var("MOONRAKER_PORT").unwrap_or("7125".to_string()).parse::<u16>().unwrap();
//     // let mut connection = MoonConnection::new(url.to_string(), 1000, 1000).await;
//     let mut connection = FastMoonConn::new(hostname, port, None, None, false).await;
//     // let status = connection.get_printer_info(Some(1234)).await.unwrap();
//     match connection.get_printer_info().await {
//         Ok(printer_info) => {
//             let state = printer_info.state;
//             println!("Printer State: {:?}", state);
//         },
//         Err(e) => {
//             eprintln!("Error getting printer info: {}", e.to_string());
//             assert!(false);
//         }
//     }
// }
