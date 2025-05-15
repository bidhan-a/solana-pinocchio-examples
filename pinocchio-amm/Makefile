build:
	cargo build-sbf

test:
	cargo test --features test

get-program-key:
	solana address -k target/deploy/pinocchio_amm-keypair.json

deploy:
	solana program deploy ./target/deploy/pinocchio_amm.so --program-id ./target/deploy/pinocchio_amm-keypair.json