all: list

install:
	python3 -m venv .venv
	. .venv/bin/activate; pip install jinja2-cli

create-dist:
	rm -rf dist
	mkdir dist
	cp src/* dist/
	cp -r ../benchmarking-tools/node_exporter dist/
	rsync -a --exclude='target/' ../benchmarking-tools/high_frequency_exporter/ dist/high_frequency_exporter/

gen-templates:
	. .venv/bin/activate; \
	jinja2 dist/entrypoint_router.sh.j2 scenarii/$(SCENARIO).json > dist/entrypoint_router.sh; \
	jinja2 dist/docker-compose.yml.j2 scenarii/$(SCENARIO).json > dist/docker-compose.yml

compose-up:
	docker compose -f dist/docker-compose.yml up --build -d

logs:
	docker compose -f dist/docker-compose.yml logs -n 5 -f

up: create-dist gen-templates compose-up

down:
	docker compose -f dist/docker-compose.yml down

clean:
	docker compose -f dist/docker-compose.yml down -v

list: scenarii/*.json
	@echo "Usage: make up SCENARIO=<name>"

scenarii/*.json:
	@echo -n $(basename $(notdir $@))" "
	@cat $@ | jq ".description"

.PHONY: create-dist gen-templates compose-up up logs install down clean all list scenarii/*.json
