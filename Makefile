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

