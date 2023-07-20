build:
	docker compose build
	docker compose run --rm  app sh -c "cargo build"
up:
	docker compose up -d $(ARGS)
restart:
	docker compose stop $(ARGS)
	docker compose rm -f $(ARGS)
	docker compose up -d $(ARGS)
down:
	docker compose down --rmi all --volumes --remove-orphans
ps:
	docker compose ps
log:
	docker compose logs -f $(ARGS)
sh:
	docker compose run --rm app sh -c "$(ARGS)"
init:
	docker compose exec app bash
build-mac:
	cargo build --release --target aarch64-apple-darwin --bin gpt-cli
	cd target/aarch64-apple-darwin/release/ && tar -czf gpt-cli-$(V)-aarch64-apple-darwin.tar.gz gpt-cli && shasum -a 256 gpt-cli-$(V)-aarch64-apple-darwin.tar.gz
build-win:
	cargo build --release --target x86_64-pc-windows-gnu  --bin gpt-cli
	cd target/x86_64-pc-windows-gnu/release/ && tar -czf gpt-cli-$(V)-x86_64-pc-windows-gnu.tar.gz gpt-cli.exe