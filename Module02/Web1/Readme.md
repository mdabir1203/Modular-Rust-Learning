# Actix Web Server

This project is a simple web server built with Actix Web that serves static HTML files and provides a basic route.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
2. Clone this repository:

    ```sh
    git clone https://github.com/yourusername/actix-web-server.git
    ```

3. Navigate to the project directory:

    ```sh
    cd actix-web-server
    ```

4. Build the project:

    ```sh
    cargo build
    ```

## Usage

To run the server, use the following command:

```sh
cargo run
```

The server will start and listen on `http://localhost:8080`. You can access the following endpoints:

- `/`: Serves a static HTML file.

## Static Files

The server serves the following static HTML files:

- `404.html`: Custom 404 error page.
- `test.html`: A test HTML file.

## Testing

To run the tests, use the following command:

```sh
cargo test
```

The tests cover the server routes and error handling.

## Features

- Actix Web: Utilizes the Actix Web framework to build a lightweight web server.
- Static File Serving: Serves static HTML files using Actix Web's `fs` module.
- Error Handling: Demonstrates error handling in Actix Web applications.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
