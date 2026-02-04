# Beacon 🚨
**A lightweight service monitoring tool that continuously checks the health of your HTTP APIs and exposes real-time status through a REST API.**
 🚀 Quick Start
# Clone and run
git clone <repository-url>
cd beacon
cargo run
🏗️ Architecture
Clean modular design with separate modules for configuration, scheduling, and HTTP server.
⚙️ Configuration
Configure endpoints in config.yaml:
endpoints:
  - name: "Test API"
    url: "https://api.example.com"
    interval_seconds: 30
    expected_status_range: [200, 299]
    max_response_time_ms: 1000
    timeout_second: 10
📡 API Reference
Current Endpoints:
- GET /api - Basic health check
- GET /api/users/{id} - Sample endpoint
Planned Endpoints:
- GET /health - System health status
- GET /health/services - All services status
- GET /health/services/{name} - Specific service status
🎯 Current Features
✅ Implemented:
- HTTP endpoint monitoring
- YAML configuration loading
- Async scheduler and server
- Basic error handling
📋 Planned:
- Health criteria validation
- Health status APIs
- Per-service scheduling
- Alerting system
- Extended monitoring (databases, processes)
🔄 Development Status
Current Phase: Core health logic implementation
Beacon is at 30% completion with solid foundation. Focus areas:
1. Implement health criteria validation
2. Add health status storage
3. Create health monitoring APIs
4. Enable per-service scheduling
📖 Future Vision
Extend beyond HTTP APIs to monitor databases, processes, and custom protocols with distributed deployment and alerting capabilities.
