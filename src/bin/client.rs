use observability::{Level, Observability};
use std::env;
use std::path::Path;
use std::thread;
use std::time::Duration;

const SOCKET_PATH: &str = "/tmp/socket-demo";

fn main() {
    sigpipe::reset();
    // Parse frequency from command line or use default 1Hz
    let args: Vec<String> = env::args().collect();
    let frequency = if args.len() > 1 {
        args[1].parse::<f64>().unwrap_or(1.0)
    } else {
        1.0
    };

    // Calculate sleep duration in milliseconds
    let sleep_duration = Duration::from_millis((1000.0 / frequency) as u64);

    // Initialize observability with UDS socket
    // Note: This configures observability to send metrics directly to our socket
    // For simplicity, we'll use hardcoded tags
    let obs = Observability::datadog_agent_unix(
        Path::new(SOCKET_PATH), // Unix socket path to send metrics to
        Level::Info,            // Log level
        vec![("service", "socket-demo-client"), ("component", "client")], // Tags to apply to all metrics
        100_000,                                                          // Max queued metrics
    )
    .expect("Failed to initialize observability with UDS socket");

    // Create a logger for the client
    let logger = obs.new_logger("socket_demo.client");
    logger.info(format!(
        "Client running at {:.2}Hz (interval: {:?})",
        frequency, sleep_duration
    ));

    // Create metrics we'll send periodically
    let requests_counter = obs.new_counter("socket_demo.requests_total");
    let frequency_gauge = obs.new_gauge("socket_demo.frequency_hz");
    let random_distribution = obs.new_distribution("socket_demo.random_values");

    // Set the frequency once
    frequency_gauge.update(frequency as i64);

    // Track iterations
    let mut iteration = 0;

    // Run forever
    loop {
        iteration += 1;

        // Record a request counter increment
        requests_counter.incr();

        // Generate some random metrics data (0-99)
        let random_value = (iteration % 100) as f64;
        random_distribution.record(random_value);

        // Log every 10 iterations at debug level
        if iteration % 10 == 0 {
            logger.debug(format!("Completed {} iterations", iteration));
        }

        // Sleep for the specified duration
        thread::sleep(sleep_duration);
    }
}
