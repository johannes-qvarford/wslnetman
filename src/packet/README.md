# Packet Sending Implementation

This module is responsible for sending packets (ping and HTTP over TCP) from the application.

## Approach

### Ping Implementation

1. **Primary Method**: Use the `ping` command available on the system
2. **Alternative Method**: Use raw sockets for ICMP echo requests (requires elevated privileges)
3. **Cross-platform**: Handle differences between Windows and Unix-like systems
4. **Data Format**: Extract response time, packet loss, and other statistics

### HTTP over TCP Implementation

1. **Primary Method**: Use the `reqwest` crate for HTTP requests
2. **Features**:
   - Support for HTTP/1.1 and HTTP/2
   - Support for HTTPS
   - Configurable timeouts
   - Support for custom headers
   - Support for different HTTP methods (GET, POST, etc.)

### Implementation Plan

1. Create functions for sending ping requests
2. Create functions for sending HTTP requests
3. Implement error handling for network timeouts and failures
4. Provide structured output for response data
5. Handle different protocols (IPv4, IPv6)
6. Support for customizing request parameters (timeout, headers, etc.)
7. Implement asynchronous operations to prevent UI blocking