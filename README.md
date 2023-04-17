# HowManyTokens

This Rust program provides a simple HTTP API to tokenize text using Hugging Face's tokenizer. It is primarily intended for use from languages where there aren't any nice or fast tokenizers available. The API allows you to get the token length and trim strings to a specified token length.

## Prerequisites

To use this program, you will need the following:

- Rust (latest stable version)
- Cargo (latest stable version)
- Docker (optional)

## Installation

### Building and Running without Docker

1. Clone the repository:

```bash
git clone https://github.com/josephhajduk/howmanytokens.git
cd howmanytokens
```

2. Build the project:

```bash
cargo build --release
```

3. Run the program:

```bash
cargo run --release
```

### Building and Running with Docker

1. Clone the repository:

```bash
git clone https://github.com/josephhajduk/howmanytokens.git
cd howmanytokens
```

2. Build the Docker image:

```bash
docker build -t howmanytokens .
```

3. Run the Docker container:

```bash
docker run -p 30000:30000 howmanytokens
```

By default, the program will listen on `0.0.0.0:30000` and use the `gpt2` tokenizer. You can change these settings by setting the `SERVER_ADDR` and `PRETRAINED` environment variables, respectively.

## API Endpoints

- `POST /len`: Get the token length of the input string.
- `POST /trim/<length>`: Trim the input string to the specified token length.
- `POST /trimw/<length>`: Trim the input string to the specified token length while ensuring that the last word is not cut off.

## Example Usage

```bash
# Get token length
curl -X POST -d "This is a test string." http://localhost:30000/len

# Trim string to 5 tokens
curl -X POST -d "This is a test string." http://localhost:30000/trim/5

# Trim string to 5 tokens without cutting off the last word
curl -X POST -d "This is a test string." http://localhost:30000/trimw/5
```

## License

This project is licensed under the [MIT License](LICENSE).

## Note

GPT4 wrote this MD file for me and I barely skimmed it so it could all be nonsense.
