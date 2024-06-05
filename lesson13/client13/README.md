# Homework - Example Chat Server
## Connecting to the Chat Server
To connect to the server with default settings (127.0.0.1:11111):
```bash
client
```

To connect to a server on a specified address (e.g., 10.0.0.100:12345):
```bash
client 10.0.0.100:12345
```

## Usage
The client supports three commands in addition to sending text messages:
- Send a text message:
```text
sample text
```
- Send a file:
```text
.file file_to_send.example
```
- Send an image: (images are converted to PNG on client, to lower the traffic and load on server)
```text
.image image_to_send.jpg
```
- Quit the client:
```text
.quit
```
