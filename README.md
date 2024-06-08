# rust_09_networking
simple chat application with file sending capabilities

UPDATE 3 - HW lesson 13, added png conversion with image crate, server error handling and notifications to clients

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

- all messages are delivered to all clients except the one which sent it
- when client is connected, server send message to all clients, that new client has connected
- when client is disconnected, server displays a message about disconnecting
- when no client is left, server shuts down too
- when .image type of message is sent, server validates if the file is image and tries to convert it to .png format. If it failes (either file is not an image or image is corrupted), it displays message to all clients, that file is in wrong format or corrupted. For the conversion, image crate is used
- files test.zip and kocka.jpg are ready for testing in root folder, 
so command .image kocka.jpg will send the image and 
.file test.zip will send the file.
- calling .image test.zip will produce server error which is displayed on all clients

I have also tried rayon crate for making the sending messages to all clients in parallel, but it didn't work, complained about HashMap iter_par() is not iterator. When used .map(|(addr, &stream)| { send_message(); }); it actually never sent the messages, as it would hang inside. Didn't know what happened.
