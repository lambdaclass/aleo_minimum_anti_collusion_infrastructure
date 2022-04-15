init:
	cargo install leo-lang
	
build:
	cargo build

ops:
	redis-server --daemonize yes

stop:
	redis-cli shutdown	

run_server:
	cargo run -p aleo-maci-server	

test:
	cargo test

run_docker:
	docker compose up

build_images:
	docker compose build

format:
	cargo fmt --all

build_circuits:
	cd circuits/tally;leo build

run_circuits:
	cd circuits/tally;leo run

clean_circuits:
	cd circuits/tally;leo clean

