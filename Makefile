COMPOSE := docker compose
SERVICE := machine-dev
BUILD_DIR := build/container

.PHONY: up down shell configure build run rebuild logs

up:
	$(COMPOSE) up -d --build

down:
	$(COMPOSE) down

shell:
	$(COMPOSE) exec $(SERVICE) sh

configure:
	$(COMPOSE) exec $(SERVICE) cmake -S . -B $(BUILD_DIR)

build:
	$(COMPOSE) exec $(SERVICE) cmake --build $(BUILD_DIR)

run:
	$(COMPOSE) exec $(SERVICE) ./$(BUILD_DIR)/machine

rebuild:
	$(COMPOSE) exec $(SERVICE) sh -lc "cmake -S . -B $(BUILD_DIR) && cmake --build $(BUILD_DIR) && ./$(BUILD_DIR)/machine"

logs:
	$(COMPOSE) logs -f $(SERVICE)
