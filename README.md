# Goal
Investigating sigpipe handling, this is a minimal repro for a larger issue that
presents itself when a rust lib is embedded into a go application.

Note the usage of `sigpipe` crate here to restore the OS default behavior, this
is done because rust's pre-amble (before `main`) ignores sigpipe.

## Setup:
1. In `./Cargo.toml`, update the `path =` to point to your local checkout of
   dd-source
2. In `dd-source`, check out `sopell/allow-local-statsd` to get local UDS-stream
   support

## Usage
This has a "client" program that uses the observability crate to send to a
unix-domain socket in stream-mode that the "server" program listens on (and
dumps some stuff out to stdout).

The goal here is to test the behavior of the observability client library when
the connection with the UDS-stream is severed.

### Experiment #1
Run `cargo run --bin client` in one terminal, `cargo run --bin server` in
another terminal, and observe that the client successsfully connects and begins
sending to the server.

Now, kill the server and observe that the client program dies with a SIGPIPE.

### Experiment #2
In `dd-source`, checkout the branch `sopell/obs-uds-nosigpipe`.

Run `cargo run --bin client` in one terminal, `cargo run --bin server` in
another terminal, and observe that the client successsfully connects and begins
sending to the server.

Now, kill the server and observe that the client is unaffected, it does not die.



