# Actix Web GitHub Repo Info

This project is a simple web service built with Actix Web that fetches information about GitHub repositories.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
2. Clone this repository:

    ```sh
    git clone https://github.com/yourusername/actix-web-github-repo-info.git
    ```

3. Navigate to the project directory:

    ```sh
    cd actix-web-github-repo-info
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

- `/repo/{owner}/{repo}`: Fetches information about the specified GitHub repository.

## Testing

To run the tests, use the following command:

```sh
cargo test
```

The tests cover the server routes and error handling.

## Features

- Actix Web: Utilizes the Actix Web framework to build a lightweight web service.
- GitHub API: Fetches repository information using the GitHub API.
- Error Handling: Demonstrates error handling in Actix Web applications.

## How It Works

### Actix Web Server

The Actix Web server is configured to listen on `http://localhost:8080`. It defines a single route that accepts two path parameters: `{owner}` and `{repo}`. When a request is made to this route, the server fetches information about the specified GitHub repository using the GitHub API.

### GitHub API Integration

The server makes a request to the GitHub API to retrieve information about the specified repository. It parses the JSON response and returns relevant details such as the repository name, description, stars, forks, and the URL.

### Error Handling

The application handles errors gracefully by returning appropriate HTTP status codes and error messages when encountering issues such as invalid repository names or failed API requests.

## Contributing

Contributions to this project are welcome! Whether it's submitting bug reports, suggesting enhancements, or contributing code, all forms of collaboration are appreciated.

## License

This project is licensed under the MIT License. See the [MIT License](https://opensource.org/licenses/MIT) for details.

Happy coding, and may the Actix be with you! 🦀✨
