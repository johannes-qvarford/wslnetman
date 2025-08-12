slint::include_modules!();
use slint::Model;

// Import packet sending modules
mod packet;
use packet::{send_http_request, send_ping};

// Import network modules
mod network;
use network::{
    filter_ports_for_interface, get_active_ports, get_all_docker_networks,
    get_all_network_interfaces,
};

/// Refresh all data for the application
fn refresh_all_data(app_weak: &slint::Weak<MainWindow>) {
    let app = app_weak.unwrap();
    println!("Refreshing data...");

    // Refresh network interfaces
    match get_all_network_interfaces() {
        Ok(interfaces) => {
            // Convert to Slint-compatible format
            let slint_interfaces: Vec<slint_generatedMainWindow::NetworkInterface> = interfaces
                .into_iter()
                .map(|interface| slint_generatedMainWindow::NetworkInterface {
                    name: interface.name.into(),
                    ipv4_addresses: interface
                        .ipv4_addresses
                        .iter()
                        .map(|ip| ip.clone().into())
                        .collect::<Vec<_>>()
                        .as_slice()
                        .into(),
                    ipv6_addresses: interface
                        .ipv6_addresses
                        .iter()
                        .map(|ip| ip.clone().into())
                        .collect::<Vec<_>>()
                        .as_slice()
                        .into(),
                    mac_address: interface
                        .mac_address
                        .unwrap_or_else(|| "N/A".to_string())
                        .into(),
                    is_up: interface.is_up,
                    is_loopback: interface.is_loopback,
                    environment: match interface.environment {
                        network::NetworkEnvironment::Windows => "Windows".into(),
                        network::NetworkEnvironment::Wsl => "WSL".into(),
                    },
                })
                .collect();

            app.set_network_interfaces(slint_interfaces.as_slice().into());
        }
        Err(e) => {
            eprintln!("Error getting network interfaces: {e}");
        }
    }

    // Refresh ports
    match get_active_ports() {
        Ok(ports) => {
            // Convert to Slint-compatible format
            let slint_ports: Vec<slint_generatedMainWindow::PortInfo> = ports
                .into_iter()
                .map(|port| slint_generatedMainWindow::PortInfo {
                    process_id: port.process_id.into(),
                    process_name: port.process_name.into(),
                    protocol: port.protocol.into(),
                    port: port.port.into(),
                    direction: port.direction.into(),
                    network: port.network.into(),
                })
                .collect();

            app.set_ports(slint_ports.as_slice().into());
        }
        Err(e) => {
            eprintln!("Error getting active ports: {e}");
        }
    }

    // Refresh Docker networks
    match get_all_docker_networks() {
        Ok(networks) => {
            // Convert to Slint-compatible format
            let slint_networks: Vec<slint_generatedMainWindow::DockerNetwork> = networks
                .into_iter()
                .map(|network| slint_generatedMainWindow::DockerNetwork {
                    name: network.name.into(),
                    driver: network.driver.into(),
                    scope: network.scope.into(),
                    subnet: network.subnet.into(),
                })
                .collect();

            app.set_docker_networks(slint_networks.as_slice().into());
        }
        Err(e) => {
            eprintln!("Error getting Docker networks: {e}");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = MainWindow::new()?;

    // Load initial data on startup
    let app_weak = app.as_weak();
    refresh_all_data(&app_weak);

    // Set up callbacks
    let app_weak = app.as_weak();
    app.on_refresh_data(move || {
        refresh_all_data(&app_weak);
    });

    let app_weak = app.as_weak();
    app.on_network_selected(move |index| {
        let _app = app_weak.unwrap();
        // Handle network selection
        println!("Network selected: {index}");
        // TODO: Implement actual network selection handling
    });

    // Handle network row clicks for detail popup
    let app_weak = app.as_weak();
    app.on_network_row_clicked(move |index| {
        let app = app_weak.unwrap();
        println!("Network row clicked: {index}");

        // Get current network interfaces
        let network_interfaces = app.get_network_interfaces();
        let ports = app.get_ports();

        if let Some(selected_interface) = network_interfaces.iter().nth(index as usize) {
            // Convert Slint NetworkInterface back to our NetworkInterface for filtering
            let rust_interface = network::NetworkInterface {
                name: selected_interface.name.to_string(),
                ipv4_addresses: selected_interface
                    .ipv4_addresses
                    .iter()
                    .map(|ip| ip.to_string())
                    .collect(),
                ipv6_addresses: selected_interface
                    .ipv6_addresses
                    .iter()
                    .map(|ip| ip.to_string())
                    .collect(),
                mac_address: if selected_interface.mac_address == "N/A" {
                    None
                } else {
                    Some(selected_interface.mac_address.to_string())
                },
                is_up: selected_interface.is_up,
                is_loopback: selected_interface.is_loopback,
                environment: match selected_interface.environment.as_str() {
                    "Windows" => network::NetworkEnvironment::Windows,
                    _ => network::NetworkEnvironment::Wsl,
                },
            };

            // Convert Slint ports to Rust ports for filtering
            let rust_ports: Vec<network::PortInfo> = ports
                .iter()
                .map(|port| network::PortInfo {
                    process_id: port.process_id.to_string(),
                    process_name: port.process_name.to_string(),
                    protocol: port.protocol.to_string(),
                    port: port.port.to_string(),
                    direction: port.direction.to_string(),
                    network: port.network.to_string(),
                })
                .collect();

            // Filter ports for this interface
            let filtered_ports = filter_ports_for_interface(&rust_interface, &rust_ports);

            // Convert filtered ports back to Slint format
            let slint_filtered_ports: Vec<slint_generatedMainWindow::PortInfo> = filtered_ports
                .into_iter()
                .map(|port| slint_generatedMainWindow::PortInfo {
                    process_id: port.process_id.into(),
                    process_name: port.process_name.into(),
                    protocol: port.protocol.into(),
                    port: port.port.into(),
                    direction: port.direction.into(),
                    network: port.network.into(),
                })
                .collect();

            // Set the selected network details and filtered ports
            app.set_selected_network_detail(selected_interface.clone());
            app.set_filtered_ports(slint_filtered_ports.as_slice().into());

            // Show the detail popup
            app.set_show_network_detail(true);
        }
    });

    let app_weak = app.as_weak();
    app.on_send_packet(move || {
        let app = app_weak.unwrap();
        // Handle packet sending
        let packet_type = app.get_packet_type();
        let destination = app.get_destination();

        // Clone values for async context
        let destination_clone = destination.clone();
        let app_weak_clone = app.as_weak();

        // Spawn async task for packet sending
        slint::spawn_local(async move {
            let app = app_weak_clone.unwrap();

            if packet_type == "ping" {
                match send_ping(&destination_clone, Some(4), Some(5)) {
                    Ok(result) => {
                        // Format ping response
                        let ping_response = format!(
                            r#"
                            Destination: {}
                            Transmitted: {}
                            Received: {}
                            Packet Loss: {:.2}%
                            Min Time: {:.2}ms
                            Avg Time: {:.2}ms
                            Max Time: {:.2}ms"#,
                            result.destination,
                            result.transmitted,
                            result.received,
                            result.packet_loss,
                            result.min_time,
                            result.avg_time,
                            result.max_time
                        );

                        // Set ping response
                        app.set_ping_response(ping_response.into());

                        // Set raw output
                        app.set_raw_output(result.raw_output.into());
                    }
                    Err(e) => {
                        let error_msg = format!("Error sending ping: {e}");
                        app.set_ping_response(error_msg.clone().into());
                        app.set_raw_output(error_msg.into());
                    }
                }
            } else if packet_type == "HTTP over TCP" {
                match send_http_request(&destination_clone, Some("GET"), Some(30)).await {
                    Ok(result) => {
                        // Format HTTP response
                        let http_response = format!(
                            "URL: {}\nStatus Code: {}\nResponse Time: {:.2}ms\n\nHeaders:\n{}\n\nBody:\n{}",
                            result.url,
                            result.status_code,
                            result.response_time,
                            result.response_headers,
                            result.response_body
                        );

                        // Set HTTP response
                        app.set_http_response(http_response.into());

                        // Set raw output
                        app.set_raw_output(result.raw_output.into());
                    }
                    Err(e) => {
                        let error_msg = format!("Error sending HTTP request: {e}");
                        app.set_http_response(error_msg.clone().into());
                        app.set_raw_output(error_msg.into());
                    }
                }
            }
        }).unwrap();
    });

    app.run()?;
    Ok(())
}
