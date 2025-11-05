
install:
	sudo apt install python3.12-venv
	python3 -m venv .venv
	. .venv/bin/activate; pip install jinja2-cli

gen-templates:
	. .venv/bin/activate; \
	jinja2 prometheus.yml.j2 template_vars.json > prometheus.yml; \
	jinja2 docker-compose.yml.j2 template_vars.json > docker-compose.yml

compose-up:
	docker compose up --build -d

logs:
	docker compose logs -n 5 -f

up: gen-templates compose-up

down:
	docker compose down


.PHONY: gen-templates compose-up up logs install down
