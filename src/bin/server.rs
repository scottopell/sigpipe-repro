use std::convert::TryInto;
use std::fs;
use std::io::{self, Read};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

fn main() {
    run_server("/tmp/socket-demo").unwrap_or_else(|e| {
        eprintln!("Server error: {}", e);
    });
}

/// Handle a connected client by reading data in chunks and discarding it
fn handle_client(mut stream: UnixStream) -> io::Result<()> {
    let mut buffer = [0; 4096];

    // Read data in chunks and discard it
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection was closed
                break;
            }
            Ok(n) => {
                // Log the received data (first 20 bytes) and total length
                let preview = if n > 20 { 20 } else { n };

                // Interpret first 4 bytes as little-endian data length
                let length_bytes = if n >= 4 {
                    let bytes: [u8; 4] = buffer[0..4].try_into().unwrap();
                    Some(u32::from_le_bytes(bytes))
                } else {
                    None
                };

                if let Some(length) = length_bytes {
                    // Create preview excluding the first 4 bytes
                    let content_preview_size = if n > 24 { 20 } else { n - 4 };
                    let content_preview = &buffer[4..4 + content_preview_size];
                    let utf8_preview = String::from_utf8_lossy(content_preview);
                    println!("Received {} bytes. Msg length: {}. First {:?} bytes (excluding length): \"{}\"",
                             n, length, content_preview_size, utf8_preview);
                } else {
                    let data_preview = &buffer[0..preview];
                    let utf8_preview = String::from_utf8_lossy(data_preview);
                    println!(
                        "Received {} bytes. First {:?} bytes: \"{}\"",
                        n, preview, utf8_preview
                    );
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

/// Run the Unix domain socket server
pub fn run_server(socket_path: &str) -> io::Result<()> {
    // Remove socket file if it already exists
    let path = Path::new(socket_path);
    if path.exists() {
        fs::remove_file(path)?;
    }

    // Create the Unix domain socket listener
    let listener = UnixListener::bind(socket_path)?;
    println!("Server listening on {}", socket_path);

    // Accept connections and handle them
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established");
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
