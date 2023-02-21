test:
	cargo test --workspace

lint:
	cargo fmt
	cargo clippy --workspace

dev:
	docker run --rm -p 5672:5672 rabbitmq:3
