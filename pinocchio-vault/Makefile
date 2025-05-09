build:
	cargo build-sbf

test:
	cargo test --features test

get-program-key:
	solana address -k target/deploy/pinocchio_vault-keypair.json

deploy:
	solana program deploy ./target/deploy/pinocchio_vault.so --program-id ./target/deploy/pinocchio_vault-keypair.json