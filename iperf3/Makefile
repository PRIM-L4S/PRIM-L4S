
install:
	sudo apt install python3.12-venv
	python3 -m venv .venv
	. .venv/bin/activate; pip install jinja2-cli

create-dist:
	rm -rf dist
	mkdir dist
	cp src/* dist/
	cp -r iperf3_exporter dist/

gen-templates:
	. .venv/bin/activate; \
	jinja2 dist/prometheus.yml.j2 template_vars.json > dist/prometheus.yml; \
	jinja2 dist/docker-compose.yml.j2 template_vars.json > dist/docker-compose.yml

compose-up:
	docker compose -f dist/docker-compose.yml up --build -d

logs:
	docker compose -f dist/docker-compose.yml logs -n 5 -f

up: create-dist gen-templates compose-up

down:
	docker compose -f dist/docker-compose.yml down

clean:
	docker compose -f dist/docker-compose.yml down -v

.PHONY: create-dist gen-templates compose-up up logs install down clean
