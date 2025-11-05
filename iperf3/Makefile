
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

start: gen-templates compose-up


.PHONY: gen-templates compose-up start logs install
