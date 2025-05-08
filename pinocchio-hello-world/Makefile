build:
	cargo build-sbf

test:
	cargo test

get-program-key:
	solana address -k target/deploy/pinocchio_hello_world-keypair.json

deploy:
	solana program deploy ./target/deploy/pinocchio_hello_world.so --program-id ./target/deploy/pinocchio_hello_world-keypair.json