# BurgasBusMS – Rust Microservice 

This microservice, BurgasBusMS, is a backend API that wraps the BurgasBusAPI. Built using Rust with the Actix-web framework, it handles requests related to real-time public transport data, such as bus schedules, routes, and current locations. This service is a key part of the real-time transport tracking system that powers the Wise (Legacy) frontend.

Part of the project that won 1st place at the PHEE TECH Hackathon.

## Features 
- Handles real-time transport data retrieval and processing 
- CORS support for frontend communication 
- Asynchronous requests with **Tokio** for improved performance 
- Integrated with external APIs (e.g., **TransportBurgas**) via **Reqwest** 

## 🛠 Stack 
**Rust, Actix-web, Tokio, Reqwest, Serde, Chrono** 

## 🚀 Startup Guide 

1. **Clone the repository:** 
   ```bash 
   git clone https://github.com/your-repo/BurgasBusMS.git 
   cd BurgasBusMS 
   ``` 

2. **Install dependencies:** 
   ```bash 
   cargo build 
   ``` 

4. **Start the microservice:** 
   ```bash 
   cargo run 
   ```

7. **Monitor logs:** 
   - Logs are handled by **env_logger**. Ensure the `RUST_LOG` environment variable is set for different log levels. 

   Example: 
   ```bash 
   export RUST_LOG=info 
   cargo run 
   ``` 

The microservice will be accessible at `http://localhost:8080` by default. 

## 📡 API Communication 
The microservice handles API requests through Actix-web endpoints, processes them, and communicates with external services using **Reqwest**. The service exposes endpoints for fetching transport data, bus routes, and live updates. 

## 🧪 Testing 
- The microservice includes unit tests to ensure data processing and API communication work correctly. You can run tests using `cargo test`. 

You can also explore the frontend of this project here: [WiseFrontendLegacy](https://github.com/ForceFinity/WiseFrontendLegacy).
