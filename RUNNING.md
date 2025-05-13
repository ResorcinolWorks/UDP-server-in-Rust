# Quick Guide: Running UDP Server and Client

## Configuration
The server and client can be configured using environment variables:
```bash
# Server configuration
export SERVER_IP="127.0.0.1"  # Default if not set
export SERVER_PORT="8080"     # Default if not set

# For production, use your actual server IP and port
# export SERVER_IP="your.server.ip"
# export SERVER_PORT="your_port"
```

## Running in Separate Terminals

### Terminal 1 (Server)
1. Open a new terminal window/tab
2. Navigate to your project directory:
```bash
cd /path/to/UDP\ Server\ in\ Rust
```
3. (Optional) Set server configuration:
```bash
export SERVER_IP="127.0.0.1"
export SERVER_PORT="8080"
```
4. Run the server:
```bash
./server
```
5. You should see: "UDP Server listening on [IP]:[PORT]"
6. Keep this terminal open - the server will keep running

### Terminal 2 (Client)
1. Open another new terminal window/tab
2. Navigate to the same project directory:
```bash
cd /path/to/UDP\ Server\ in\ Rust
```
3. (Optional) Set client configuration to match server:
```bash
export SERVER_IP="127.0.0.1"
export SERVER_PORT="8080"
```
4. Run the client:
```bash
./client
```
5. You should see: "Connected to server at [IP]:[PORT]"

## Testing the Connection
1. In the client terminal, type any message and press Enter
2. You should see:
   - In client: "Sent: [your message]"
   - In server: "Received from [address]: [your message]"
   - In client: "Received: [your message]"

## Stopping the Applications
- To stop the client: Type 'quit' and press Enter
- To stop the server: Press Ctrl+C in the server terminal

## Security Notes
- Never commit real IP addresses to version control
- Use environment variables for configuration
- In production, use proper authentication and encryption
- Consider using a configuration file for more complex setups
- Always validate and sanitize user input

## Common Issues
- If you get "Address already in use": The server is already running somewhere
- If you get "Connection refused": Make sure the server is running first
- If messages aren't being received: Check that both terminals are in the correct directory
- If connection fails: Verify the IP and port settings match between server and client 