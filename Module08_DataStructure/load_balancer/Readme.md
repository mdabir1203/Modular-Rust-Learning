
# Rust Load Balancer

This project implements a simple load balancer in Rust. The load balancer listens for incoming TCP connections and distributes them to backend servers.

## What is a Load Balancer?

A load balancer is a device or software that distributes network or application traffic across multiple servers. By spreading the load, a load balancer ensures that no single server becomes overwhelmed, which helps to improve the responsiveness and availability of applications.

## Why is a Load Balancer Required?

1. **Improved Performance**: By distributing the workload evenly across multiple servers, a load balancer can help to improve the overall performance of an application.
2. **High Availability**: Load balancers can detect server failures and reroute traffic to healthy servers, ensuring that the application remains available even if one or more servers go down.
3. **Scalability**: As the demand for an application grows, additional servers can be added to the pool behind the load balancer, allowing the application to scale horizontally.
4. **Redundancy**: Load balancers provide redundancy by ensuring that traffic is rerouted in case of server failures, thus minimizing downtime.
5. **Security**: Load balancers can also serve as a security layer. They can help:
    - Mitigate DDoS Attacks: By distributing traffic, load balancers can absorb and mitigate distributed denial-of-service attacks.
    - SSL Termination: Handling SSL/TLS encryption at the load balancer level to offload processing from backend servers.

## Prerequisites

- Rust and Cargo installed on your system. You can install both by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Installation

1. Clone the repository:

	```bash
	git clone https://github.com/mdabir1203/InventRust.git
	```

2. Navigate to the load balancer directory:

	```bash
	cd InventRust/Module08_DataStructure/load_balancer
	```

3. Build the project:

	```bash
	cargo build --release
	```

## Usage

### Running the Load Balancer

To run the load balancer, execute the following command:

```rust
cargo run --bin load_balancer

To run the TCP Client

```rust
cargo run --bin tcp_client --release

### Remember to make sure the Backend Server is running before running the Load Balancer. To run the backend servers run the command: 

```rust
cargo run --bin backend_server PORT_Value	

### Also remember to change the port value in the TCP Client to align with the load balancer



