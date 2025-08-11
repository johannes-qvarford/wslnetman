slint::include_modules!();

// Import packet sending modules
mod packet;
use packet::{send_http_request, send_ping};

// Import network modules
mod network;
use network::{get_active_ports, get_all_docker_networks, get_all_network_interfaces};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = MainWindow::new()?;

    // Set up callbacks
    let app_weak = app.as_weak();
    app.on_refresh_data(move || {
        let app = app_weak.unwrap();
        // Refresh all data
        println!("Refreshing data...");

        // Refresh network interfaces
        match get_all_network_interfaces() {
            Ok(interfaces) => {
                // Convert to Slint-compatible format
                let slint_interfaces: Vec<slint_generatedMainWindow::NetworkInterface> = interfaces
                    .into_iter()
                    .map(|interface| slint_generatedMainWindow::NetworkInterface {
                        name: interface.name.into(),
                        ip_addresses: interface
                            .ip_addresses
                            .iter()
                            .map(|ip| ip.clone().into())
                            .collect::<Vec<_>>()
                            .as_slice()
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
    });

    let app_weak = app.as_weak();
    app.on_network_selected(move |index| {
        let _app = app_weak.unwrap();
        // Handle network selection
        println!("Network selected: {index}");
        // TODO: Implement actual network selection handling
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
                            "Destination: {}\nTransmitted: {}\nReceived: {}\nPacket Loss: {:.2}%\nMin Time: {:.2}ms\nAvg Time: {:.2}ms\nMax Time: {:.2}ms",
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
