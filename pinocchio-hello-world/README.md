# pinocchio-hello-world

This is a simple Solana program that demonstrates the use of the Pinocchio library to log a "Hello, Solana!" message.

## Project Structure

- **`src/lib.rs`**: Contains the main program logic, including the entrypoint and the `process_instruction` function.
- **`tests/tests.rs`**: Includes integration tests for the program using the `mollusk_svm` framework.

## Prerequisites

- Rust and Cargo
- Solana CLI tools
- Pinocchio
- Mollusk (for testing)

## Build the Program

To build the program, run the following command:

```bash
make build
```

## Test the Program

To test the program, run the following command:

```bash
make test
```

## Get the Program key

To get the program key, run the following command:

```bash
make test
```

Replace the key inside the `declare_id!("..")` macro with your key.


## Deploy the Program

To deploy the program, run the following command:

```bash
make deploy
```
