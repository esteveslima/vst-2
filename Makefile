#	Utility commands to setup local development environment

COMPOSE_FILE_PATH = ./assets/environment/docker-compose.yml
COMPOSE_SERVICE_NAME = rust-container
COMPOSE_PROJECT_NAME = vst-2


# clear temp files
clear:
	sudo find . -type d \( -name "target" -or -name ".volumes" \) -prune -exec rm -rf {} \;

# setup optional docker environment
up:
	docker-compose --file $(COMPOSE_FILE_PATH) --project-name $(COMPOSE_PROJECT_NAME) up --detach
clean-up:
	docker-compose --file $(COMPOSE_FILE_PATH) --project-name $(COMPOSE_PROJECT_NAME) up --detach --build --force-recreate --always-recreate-deps
down:
	docker-compose --file $(COMPOSE_FILE_PATH) --project-name $(COMPOSE_PROJECT_NAME) down
clean-down:
	docker-compose --file $(COMPOSE_FILE_PATH) --project-name $(COMPOSE_PROJECT_NAME) down --rmi all --volumes --remove-orphans

rebuild:
	make down && make up

sh:
	docker-compose --file $(COMPOSE_FILE_PATH) --project-name $(COMPOSE_PROJECT_NAME) exec --privileged $(COMPOSE_SERVICE_NAME) bash
bash:
	docker-compose --file $(COMPOSE_FILE_PATH) --project-name $(COMPOSE_PROJECT_NAME) exec --privileged $(COMPOSE_SERVICE_NAME) bash