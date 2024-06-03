# rust_09_networking
simple chat application with file sending capabilities

to run the chat do following:
1. execute
cargo run --bin server 
to start the server, to specify custom port for the server, add it as a param:
cargo run --bin server [port]

2. execute
cargo run --bin client
to start client, to specify custom address and port for the server, add them as params:
cargo run --bin client [address] [port]

[port] should be the same as in server

4. after running client, enter your name, to identify your self

5. a. either type text mesage and press enter at the end to send it
4. b. or type .text [message] to send text message (same as without .text)
              .file [path_to_file] to send binary file from local storage
              .image [path_to_image] to send the image (.jpg, .png, .gif)
              .quit to disconnect the client from the server

all messages are delivered to all clients except the one which sent it
when client is disconnected, server displays a message about disconnecting
when no client is left, server shuts down too
