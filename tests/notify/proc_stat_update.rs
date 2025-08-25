use moonsock::{MoonNotification, NotificationMethod, NotificationParam};

#[test]
fn notify_proc_stat_update() {
	let message = r##"{
        "jsonrpc": "2.0",
        "method": "notify_proc_stat_update",
        "params": [{
            "moonraker_stats": {
                "time": 1615837812.0894408,
                "cpu_usage": 1.99,
                "memory": 23636,
                "mem_units": "kB"
            },
            "cpu_temp": 44.008,
            "network": {
                "lo": {
                    "rx_bytes": 114555457,
                    "tx_bytes": 114555457,
                    "bandwidth": 2911.49
                },
                "wlan0": {
                    "rx_bytes": 48773134,
                    "tx_bytes": 115035939,
                    "bandwidth": 3458.77
                }
            },
            "system_cpu_usage": {
                "cpu": 2.53,
                "cpu0": 3.03,
                "cpu1": 5.1,
                "cpu2": 1.02,
                "cpu3": 1
            },
            "websocket_connections": 2
        }]
    }"##;

	let serialized: MoonNotification = serde_json::from_str(message).unwrap();

	let MoonNotification { method, params, .. } = serialized;

	assert_eq!(method, NotificationMethod::NotifyProcStatUpdate);

	match params.unwrap().clone() {
		NotificationParam::ProcStatUpdate(printer_object_status) => {
			let pos = printer_object_status[0].clone();
			assert_eq!(pos.clone().moonraker_stats.time, 1615837812.0894408);
			assert_eq!(pos.clone().moonraker_stats.cpu_usage, 1.99);
			assert_eq!(pos.clone().moonraker_stats.memory, 23636);
			assert_eq!(pos.clone().moonraker_stats.mem_units, "kB");
			assert_eq!(pos.clone().cpu_temp.clone(), Some(44.008));
			assert_eq!(pos.clone().network.get("lo").unwrap().rx_bytes, 114555457);
			assert_eq!(pos.clone().network.get("lo").unwrap().tx_bytes, 114555457);
			assert_eq!(pos.clone().network.get("lo").unwrap().bandwidth, 2911.49);
			assert_eq!(pos.clone().network.get("wlan0").unwrap().rx_bytes, 48773134);
			assert_eq!(pos.clone().network.get("wlan0").unwrap().tx_bytes, 115035939);
			assert_eq!(pos.clone().network.get("wlan0").unwrap().bandwidth, 3458.77);
			assert_eq!(*pos.system_cpu_usage.get("cpu").unwrap(), 2.53);
			assert_eq!(*pos.system_cpu_usage.get("cpu0").unwrap(), 3.03);
			assert_eq!(*pos.system_cpu_usage.get("cpu1").unwrap(), 5.1);
			assert_eq!(*pos.system_cpu_usage.get("cpu2").unwrap(), 1.02);
			assert_eq!(*pos.system_cpu_usage.get("cpu3").unwrap(), 1.0);
			assert_eq!(pos.websocket_connections, 2);
		}
		_ => panic!("ERROR: Did not parse params as MoonParam::NotifyProcStatUpdate"),
	}

	// match serialized {
	// 	MoonNotification { method, params, .. } => {
	// 		assert_eq!(method, NotificationMethod::NotifyProcStatUpdate);
	//
	// 		// match params.unwrap()[0].clone() {
	// 		match params.unwrap().clone() {
	// 			NotificationParam::ProcStatUpdate(printer_object_status) => {
	// 				let pos = printer_object_status[0].clone();
	// 				assert_eq!(pos.clone().moonraker_stats.time, 1615837812.0894408);
	// 				assert_eq!(pos.clone().moonraker_stats.cpu_usage, 1.99);
	// 				assert_eq!(pos.clone().moonraker_stats.memory, 23636);
	// 				assert_eq!(pos.clone().moonraker_stats.mem_units, "kB");
	// 				assert_eq!(pos.clone().cpu_temp.clone(), Some(44.008));
	// 				assert_eq!(pos.clone().network.get("lo").unwrap().rx_bytes, 114555457);
	// 				assert_eq!(pos.clone().network.get("lo").unwrap().tx_bytes, 114555457);
	// 				assert_eq!(pos.clone().network.get("lo").unwrap().bandwidth, 2911.49);
	// 				assert_eq!(pos.clone().network.get("wlan0").unwrap().rx_bytes, 48773134);
	// 				assert_eq!(pos.clone().network.get("wlan0").unwrap().tx_bytes, 115035939);
	// 				assert_eq!(pos.clone().network.get("wlan0").unwrap().bandwidth, 3458.77);
	// 				assert_eq!(*pos.system_cpu_usage.get("cpu").unwrap(), 2.53);
	// 				assert_eq!(*pos.system_cpu_usage.get("cpu0").unwrap(), 3.03);
	// 				assert_eq!(*pos.system_cpu_usage.get("cpu1").unwrap(), 5.1);
	// 				assert_eq!(*pos.system_cpu_usage.get("cpu2").unwrap(), 1.02);
	// 				assert_eq!(*pos.system_cpu_usage.get("cpu3").unwrap(), 1.0);
	// 				assert_eq!(pos.websocket_connections, 2);
	// 			}
	// 			_ => panic!("ERROR: Did not parse params as MoonParam::NotifyProcStatUpdate"),
	// 		}
	// 	}
	// 	_ => panic!("ERROR: Did not parse as MoonNotification"),
	// }
}
