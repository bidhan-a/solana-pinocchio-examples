build:
	cargo build-sbf

test:
	cargo test --features test

get-program-key:
	solana address -k target/deploy/pinocchio_escrow-keypair.json

deploy:
	solana program deploy ./target/deploy/pinocchio_escrow.so --program-id ./target/deploy/pinocchio_escrow-keypair.json