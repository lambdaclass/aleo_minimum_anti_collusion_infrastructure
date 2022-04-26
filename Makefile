init:
	cargo install leo-lang

ops:
	redis-server --daemonize yes

ops_docker:
	docker-compose up -d redis

stop:
	redis-cli shutdown

stop_docker:
	docker-compose down redis	
	
build:
	cargo build

run_server:
	HOST="localhost" REDIS_URL="redis://127.0.0.1:6379" cargo run -p aleo-maci-server

run_server_docker:
	docker-compose up server

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

send_vote:
	cargo run --bin aleo-maci-cli vote-for 1

start_tally:
	curl -X POST http://127.0.0.1:3000/election/tally/start
