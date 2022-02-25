build:
	cargo build

run_server:
	cargo run -p aleo-maci-server

run_cli:
	cargo run -p aleo-maci-cli

test:
	cargo test

run_docker:
	docker compose up

build_images:
	docker compose buildß

format:
	cargo fmt --all

