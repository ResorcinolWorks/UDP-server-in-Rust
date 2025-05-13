# Quick Guide: Running UDP Server and Client

## Running in Separate Terminals

### Terminal 1 (Server)
1. Open a new terminal window/tab
2. Navigate to your project directory:
```bash
cd /path/to/UDP\ Server\ in\ Rust
```
3. Run the server:
```bash
./server
```
4. You should see: "UDP Server listening on 127.0.0.1:8080"
5. Keep this terminal open - the server will keep running

### Terminal 2 (Client)
1. Open another new terminal window/tab
2. Navigate to the same project directory:
```bash
cd /path/to/UDP\ Server\ in\ Rust
```
3. Run the client:
```bash
./client
```
4. You should see: "Connected to server at 127.0.0.1:8080"

## Testing the Connection
1. In the client terminal, type any message and press Enter
2. You should see:
   - In client: "Sent: [your message]"
   - In server: "Received from [address]: [your message]"
   - In client: "Received: [your message]"

## Stopping the Applications
- To stop the client: Type 'quit' and press Enter
- To stop the server: Press Ctrl+C in the server terminal

## Common Issues
- If you get "Address already in use": The server is already running somewhere
- If you get "Connection refused": Make sure the server is running first
- If messages aren't being received: Check that both terminals are in the correct directory 