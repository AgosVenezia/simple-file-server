# Simple File Server

![Rust](https://pbs.twimg.com/media/GWw_eWiWIAAbbo_?format=jpg&name=small)

## Overview

The Simple File Server is a basic HTTP server written in Rust. It serves static files and directory listings over HTTP. This server can be used to explore files and directories from a specified root directory, with a focus on handling HTML files, text files, and directory listings. It does not use external HTTP libraries, adhering to the challenge requirements.

## Features

- Serves static HTML files.
- Provides directory listings with clickable links to files and subdirectories.
- Supports file serving with proper MIME types.
- Handles requests for files and directories while preventing directory traversal attacks.
- Displays a custom 404 error page for non-existent resources.
- Shows a 400 error for malformed requests.

## Project Structure

- `src/main.rs`: Main Rust source file containing the server implementation.
- `index.html`: Default HTML file served at the root URL.
- `subdir/`: Directory for testing nested directories and files.
  - Example files: `about.html`, `notes.txt`, `images/logo.png`, `styles.css`, `script.js`.

## Requirements

- Rust programming language.
- `walkdir` crate for directory traversal.
- `infer` crate for determining MIME types.
- `url-escape` crate for handling URL encoding.

## Installation

1. Ensure you have Rust installed. If not, you can install it from [https://rustup.rs](https://rustup.rs).

2. Clone the repository:
```bash
   git clone https://github.com/AgosVenezia/simple-file-server.git
   cd simple-file-server
```

3. Build the project:
```bash
   cargo build
```

## Usage

1. Navigate to the project directory and start the server:
```bash
cargo run
```

2. Open your web browser and visit `http://127.0.0.1:7878` to access the server.

3. You can explore files and directories. For example:

- View `index.html` at `http://127.0.0.1:7878/index.html`.
- Access files in `subdir/` such as `http://127.0.0.1:7878/subdir/about.html`.
- Navigate through directories and view text files or images.

## How It Works

- **Handling Requests:** The server reads incoming requests and determines the requested path. It decodes the URL and constructs the file path relative to the root directory.
- **Serving Files:** If the path is a file, the server reads the file, determines its MIME type using the `infer` crate, and sends it as the response.
- **Directory Listings:** If the path is a directory, the server generates an HTML page listing the contents of the directory, with links to each file and subdirectory.
- **Error Handling:** The server responds with a 404 Not Found error for invalid paths or resources and a 400 Bad Request error for malformed requests.

## Contributing

Contributions are welcome! If you find any bugs or have suggestions for improvements, please submit an issue or a pull request on GitHub.

#### [A project by Agostina Venezia for StackUp](https://earn.stackup.dev/)