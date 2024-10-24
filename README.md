# WebEx File Sender CLI

A command-line application written in Rust that allows you to send files through Cisco WebEx directly from your terminal.

## Authors and Contributors
Andy Dixon - [Github](https://www.github.com/andydixon) - [Web](https://www.andydixon.com/)

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Error Handling](#error-handling)
- [Dependencies](#dependencies)
- [License](#license)

## Features

- Send files to any WebEx user via email address.
- Include a custom message with the file.
- Command-line interface for quick usage.

## Prerequisites

- **Rust**: Ensure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Cisco WebEx Account**: You must have a valid Cisco WebEx account.
- **WebEx Access Token**: Obtain a personal access token to authenticate API requests.

## Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/webex-file-sender.git
   cd webex-file-sender
   ```

2. **Install Dependencies**

   The project uses `reqwest` for HTTP requests and `serde_json` for JSON handling. These dependencies are specified in the `Cargo.toml` file and will be handled automatically during the build.

3. **Build the Application**

   ```bash
   cargo build --release
   ```

   The compiled executable will be located at `target/release/webex-file-sender`.

## Configuration

Before running the application, you need to configure your WebEx access token.

1. **Obtain Access Token**

   - Log in to [WebEx for Developers](https://developer.webex.com/docs/getting-started).
   - Navigate to your profile and copy your personal access token.

2. **Set Access Token in the Code**

   - Open the `src/main.rs` file.
   - Replace `"TOKEN GOES HERE"` with your actual access token:

     ```rust
     let access_token = "YOUR_ACCESS_TOKEN";
     ```

   **Security Note**: Be careful not to expose your access token publicly. Do not commit it to version control or share it.

   **Alternative (Recommended)**: For better security, modify the code to read the access token from an environment variable:

   ```rust
   let access_token = env::var("WEBEX_ACCESS_TOKEN")
       .expect("Please set the WEBEX_ACCESS_TOKEN environment variable");
   ```

   Then, set the environment variable before running the application:

   ```bash
   export WEBEX_ACCESS_TOKEN="YOUR_ACCESS_TOKEN"
   ```

## Usage

Run the application with the following syntax:

```bash
./webex-file-sender <recipient_email> <file_path> <message_text>
```

- `<recipient_email>`: Email address of the recipient.
- `<file_path>`: Path to the file you want to send.
- `<message_text>`: The message text to accompany the file.

**Example**

```bash
./webex-file-sender jane.doe@example.com ~/Documents/report.pdf "Hi Jane, please find the report attached."
```

## Error Handling

- **File Not Found**: If the specified file path does not exist, the application will exit with an error message.
- **Invalid Recipient**: If no user is found with the given email address, an error will be displayed.
- **API Errors**: Any errors returned by the WebEx API, such as authentication errors or network issues, will be printed to stderr.

## Dependencies

- [reqwest](https://crates.io/crates/reqwest): For making HTTP requests.
- [serde_json](https://crates.io/crates/serde_json): For parsing JSON responses.

These are specified in the `Cargo.toml` file:

```toml
[package]
name = "webex-file-sender"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.11", features = ["blocking", "multipart"] }
serde_json = "1.0"
```

## License

This project is licensed under the GPLv3 License.

---

**Disclaimer**: This application is provided as-is without any warranties. Use it responsibly and ensure you comply with Cisco WebEx's terms of service.
