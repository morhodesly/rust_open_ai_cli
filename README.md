# OpenAI Chat Completion Client

This Rust program allows you to interact with OpenAI's GPT-4 model via the command line. It uses the `hyper` HTTP client along with `hyper-tls` for secure HTTPS connections and `serde_json` for JSON serialization/deserialization. The program reads user input, sends it to the OpenAI API, and prints the response.

## Features

- Interactive command-line interface.
- Sends messages to the GPT-4 model.
- Displays a spinner while waiting for the API response.
- Supports exiting the loop by typing `exit`.

## Prerequisites

1. **Rust**: Make sure you have Rust installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).
2. **OpenAI API Key**: You need an API key from OpenAI. You can obtain one by signing up at [OpenAI's website](https://beta.openai.com/signup).

## Setup

1. **Clone the repository** (if you haven't already):

   ```bash
   git clone <repository-url>
   cd <repository-directory>
2. **Create a .env file in the root directory of your project and add your OpenAI API key:**

   `OPEN_AI_TOKEN=your_openai_api_key_here`

3. **Build the project using Cargo:**

    `cargo build --release`

## Usage

Run the executable from the target/release directory:

  `./target/release/<executable-name>`

Once running, you can start typing your questions or messages. The program will send them to the GPT-4 model and print the response. Type exit to quit the loop.

## Dependencies
This project uses the following crates:

- `dotenv`: For loading environment variables from a .env file.
- `hyper` and `hyper-tls`: For making HTTPS requests.
- `serde` and `serde_derive`: For JSON serialization/deserialization.
- `spinners`: To display a spinner while waiting for the API response.
