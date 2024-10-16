sqlx:
	cargo install sqlx-cli --no-default-features --features postgres

plsq:
	apt update && apt install postgresql-client -y

get:
	http GET localhost:8000/health_check

post:
	http -f POST localhost:8000/subscriptions email=joker11@gmail.com name=joker11

run:
	cargo run | bunyan

bunyan:
	cargo install bunyan

test:
	TEST_LOG=true cargo test health_check_works | bunyan

check:
	cargo check

prepare:
	cargo sqlx prepare -- --lib

build:
	docker build -t zero2prod --file Dockerfile .

docker_run:
	docker run --network=host zero2prod

deploy:
	doctl apps create --spec spec.yaml

.PHONY: get post run test check prepare build deploy