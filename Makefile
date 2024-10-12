
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

deploy_update:
	doctl apps update c2da88f2-7431-4959-8ae2-93587751dacb --spec=spec.yaml

deploy_migrate:
	DATABASE_URL=postgresql://newsletter:AVNS_pzX_V7bHUhvA2X9Iy-p@app-b8c7f77a-dad8-41e8-8214-65539de12c8f-do-user-12086463-0.d.db.ondigitalocean.com:25060/newsletter sqlx migrate run

.PHONY: get post run test check prepare build deploy deploy_update deploy_migrate