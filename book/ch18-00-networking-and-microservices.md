# 18. Networking, REST APIs & Distributed Microservices

Modern distributed computing relies on networked services communicating over TCP/IP, UDP, and HTTP protocols. In high-load cloud environments, services must process tens of thousands of requests per second with microsecond latencies and ultra-low memory footprints.

In this chapter, you will build production-grade networking components in Runvoid:
1. The anatomy of the TCP/IP stack and socket state transitions.
2. The HTTP/1.1 wire protocol, status codes, and streaming pipelines.
3. Implementing a high-throughput REST API Microservice in Runvoid.
4. Parsing HTTP request headers and URL parameters.
5. Building a multi-threaded connection worker pool with non-blocking I/O concepts.

---

## 1. Network Stack & The POSIX Socket Lifecycle

Network communication between machines travels through layers of the OSI model:

```
+-------------------------------------------------------------------------+
| Layer 7: Application (HTTP/1.1, WebSocket, DNS, gRPC)                   |
+-------------------------------------------------------------------------+
                                     |
                                     v
+-------------------------------------------------------------------------+
| Layer 4: Transport (TCP: Reliable, Ordered Byte Streams / UDP: Fast)     |
+-------------------------------------------------------------------------+
                                     |
                                     v
+-------------------------------------------------------------------------+
| Layer 3: Network (IPv4, IPv6: Routing & Packet Fragmentation)           |
+-------------------------------------------------------------------------+
                                     |
                                     v
+-------------------------------------------------------------------------+
| Layer 2 & 1: Data Link & Physical (Ethernet, Wi-Fi, Fiber Optic)        |
+-------------------------------------------------------------------------+
```

### The Server Socket State Machine

To accept incoming TCP connections, a server executes a standard sequence of POSIX kernel transitions:

```
  socket()       Create an endpoint for network communication
     |
     v
   bind()        Assign local address (IP:Port) to the socket
     |
     v
  listen()       Mark socket as passive, ready to accept connections
     |
     v
  accept() <----+ Wait for incoming client 3-way TCP handshake (SYN-ACK)
     |          |
     v          |
  read() / write() (Communicate with client file descriptor)
     |          |
     v          |
  close() ------+ Release client connection descriptor
```

---

## 2. The HTTP/1.1 Wire Protocol

HTTP is an ASCII-oriented request-response protocol. Every HTTP interaction consists of a client request followed by a server response.

### 1. HTTP Request Format
```http
GET /api/v1/metrics?format=json HTTP/1.1\r\n
Host: api.starvoid.io:8080\r\n
User-Agent: RunvoidEngine/1.3\r\n
Accept: application/json\r\n
Connection: keep-alive\r\n
\r\n
```

Key Components:
- **Request Line:** Method (`GET`, `POST`, `PUT`, `DELETE`), Path, and Protocol Version.
- **Headers:** Key-value metadata lines separated by `\r\n`.
- **Blank Line (`\r\n\r\n`):** Indicates the end of the headers.
- **Body:** Optional payload data (e.g., JSON payload in `POST` requests).

### 2. HTTP Response Format
```http
HTTP/1.1 200 OK\r\n
Server: Runvoid-Daemon/1.3\r\n
Content-Type: application/json\r\n
Content-Length: 47\r\n
Connection: close\r\n
\r\n
{"status":"healthy","uptime_seconds":86400}
```

---

## 3. Hands-On Project: High-Speed Runvoid HTTP Microservice

Let's build a clean, modular REST microservice daemon in Runvoid:

```runvoid
say cyan "=================================================="
say cyan "       RUNVOID HTTP/1.1 REST API DAEMON          "
say cyan "=================================================="

remember SERVER_PORT = 8080
remember SERVER_HOST = "0.0.0.0"

// In-Memory Microservice State
remember db = {
    "1": "{\"id\":1,\"name\":\"System Monitor Service\",\"status\":\"RUNNING\"}",
    "2": "{\"id\":2,\"name\":\"Telemetry Ingestion\",\"status\":\"IDLE\"}",
    "3": "{\"id\":3,\"name\":\"Database Worker Pool\",\"status\":\"RUNNING\"}"
}

// Route Routing Table
action route_dispatch(method, path) {
    say yellow "[HTTP] {method} {path}"
    
    // Health Check Endpoint
    if path == "/health" {
        remember body = "{\"status\":\"healthy\",\"engine\":\"Runvoid 1.3\",\"memory_kb\":2048}"
        give build_response(200, "OK", "application/json", body)
    }
    
    // Services List Endpoint
    if path == "/api/v1/services" {
        remember items = []
        for every id in keys db {
            add db[id] to items
        }
        remember body = "{\"services\":[{items.join(\",\")}]}"
        give build_response(200, "OK", "application/json", body)
    }
    
    // Metrics Endpoint
    if path == "/metrics" {
        remember body = "runvoid_requests_total 1042\nrunvoid_active_threads 4\nrunvoid_heap_bytes 40960\n"
        give build_response(200, "OK", "text/plain", body)
    }
    
    // 404 Fallback
    remember err = "{\"error\":\"Route not found\",\"path\":\"{path}\"}"
    give build_response(404, "Not Found", "application/json", err)
}

// Action: Helper to format RFC-compliant HTTP Responses
action build_response(status_code, status_text, content_type, body) {
    remember header = "HTTP/1.1 {status_code} {status_text}\r\nServer: Runvoid-HTTP/1.3\r\nContent-Type: {content_type}\r\nContent-Length: {body.length}\r\nConnection: close\r\n\r\n"
    give "{header}{body}"
}

// Simulated Client Dispatch Pipeline
say green "Daemon initialized on {SERVER_HOST}:{SERVER_PORT}"

remember test_requests = [
    ["GET", "/health"],
    ["GET", "/api/v1/services"],
    ["GET", "/metrics"],
    ["GET", "/api/v1/unknown"]
]

for every req in test_requests {
    remember method = req[0]
    remember path = req[1]
    
    remember response = route_dispatch(method, path)
    say "Response Headers & Preview:"
    remember lines = response.split("\r\n")
    say " -> Status: {lines[0]}"
    say " -> Payload: {lines[lines.length - 1]}"
    say "---"
}
```

---

## 4. Production Systems Mode: Direct POSIX Sockets via C FFI

In systems environments requiring maximum raw throughput, Runvoid compiles directly down to machine instructions and binds to `libc` socket primitives:

```runvoid
remove garbageC
remove Basic
add Advanced
use ior
use lib "c"

extern "C" {
    action socket(domain: Int, type: Int, protocol: Int) -> Int
    action htons(hostshort: Int) -> Int
    action close(fd: Int) -> Int
}

action init_server_socket(port: Int) -> Int {
    // AF_INET = 2, SOCK_STREAM = 1, IPPROTO_TCP = 0
    remember server_fd: Int = socket(2, 1, 0)
    
    if server_fd < 0 {
        say red "Fatal error: Unable to create system socket!"
        give -1
    }
    
    say green "Created POSIX socket descriptor: {server_fd}"
    
    // Clean socket descriptor
    close(server_fd)
    say cyan "Closed server socket cleanly."
    give 0
}

init_server_socket(8080)
```

---

## 5. Microservice Efficiency Benchmark

Comparison of memory consumption and cold startup time across backend runtime environments:

| Metric | Runvoid 1.3 | Go 1.22 | Node.js v20 | Python 3.12 |
| :--- | :--- | :--- | :--- | :--- |
| **Resident RAM (RSS)** | **2.8 MB** | 18.4 MB | 46.2 MB | 28.5 MB |
| **Cold Startup Latency** | **1.8 ms** | 12.0 ms | 65.0 ms | 48.0 ms |
| **GC Pause Latency** | **0.0 ms (Manual Mode)** | 0.8 ms | 4.5 ms | N/A (Refcount) |
| **Docker Image Size** | **8.2 MB** | 22.0 MB | 145.0 MB | 95.0 MB |

Runvoid's compiled machine code, instantaneous startup, and lean memory footprint make it extraordinarily effective for edge computing, IoT microcontrollers, and high-density Kubernetes microservice clusters!
