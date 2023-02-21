test:
	cargo test --workspace

lint:
	cargo fmt
	cargo clippy --workspace

dev:
	docker run --rm -p 5672:5672 -p 15672:15672 rabbitmq:3-management
