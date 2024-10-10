
http:
	curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions

http2:
	http -f POST localhost:8000/subscriptions email=joker11@gmail.com name=joker11

run:
	cargo run | bunyan

bunyan:
	cargo install bunyan

test:
	TEST_LOG=true cargo test health_check_works | bunyan

check:
	cargo check

.PHONY: http http2 run test check