slint::include_modules!();

// Import packet sending modules
mod packet;
use packet::{send_http_request, send_ping};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = MainWindow::new()?;

    // Set up callbacks
    let app_weak = app.as_weak();
    app.on_refresh_data(move || {
        let _app = app_weak.unwrap();
        // Refresh all data
        println!("Refreshing data...");
        // TODO: Implement actual data refresh
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
