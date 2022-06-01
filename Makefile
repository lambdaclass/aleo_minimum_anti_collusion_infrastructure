init:
	cargo install leo-lang

nix_shell:
	nix-shell

build:
	@echo "👷 * Aleo Maci building process started *"
	@echo "========================================="
	@echo "🔨 1/3 Building the Server ..."""
	cargo build --release -p aleo-maci-server
	@echo "🔨 2/3 Building the CLI project..."
	cargo build --release -p aleo-maci-cli
	@echo "🔨 3/3 Building the Dashboard..."
	npm install --prefix dashboard --silent 
	npm run re:build --prefix dashboard
	@echo "==================================================="
	@echo "✅ Aleo Maci building process finished sucessfully!"

ops:
	redis-server --daemonize yes

stop_ops:
	redis-cli shutdown

ops_docker:
	docker-compose up -d redis

stop:
	redis-cli shutdown

stop_docker:
	docker-compose down redis	

run_server:
	HOST="localhost" REDIS_URL="redis://127.0.0.1:6379" cargo run --release -p aleo-maci-server

run_dashboard:
	npm run re:build --prefix dashboard
	PORT=4000 REACT_APP_MACI_HOST=http://127.0.0.1:3000 npm start --prefix dashboard

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
	cd circuits/whitelist;leo build

run_tally_circuit:
	cd circuits/tally;leo run

run_whitelist_circuit:
	cd circuits/whitelist;leo run

clean_circuits:
	cd circuits/tally;leo clean

send_test_vote_1:
	cargo run --release --bin aleo-maci-cli vote-for 1 APrivateKey1zkpFgVh5ptpLgH39p2StPmRzAu14XPat415vRv6XmcR2Sun http://localhost:3000

send_test_vote_2:
	cargo run --release --bin aleo-maci-cli vote-for 2 APrivateKey1zkpJJGbEaWCqv2ASPMRVuT44D2EWKQArHoDCzyTbwq6CFVk http://localhost:3000

send_test_vote_3:
	cargo run --release --bin aleo-maci-cli vote-for 3 APrivateKey1zkpCmLmroWhvXj37G5h1LoS4SEbK3f2zHAt2Qk8tVpB5BBx http://localhost:3000

send_vote:
	cargo run --release --bin aleo-maci-cli vote-for $(option) $(account) $(server)

start_tally:
	curl -X POST http://127.0.0.1:3000/election/tally/start

create_test_tally:
	curl --location --request POST 'http://127.0.0.1:3000/election/whitelist' --header 'content-type: application/json' -d @./client/data/whitelist.json
