COMPOSE := docker compose
SERVICE := machine-dev
BUILD_DIR := build/container
LOCAL_BUILD_DIR := /tmp/machine-build
VCPKG_ROOT ?= C:/vcpkg

.PHONY: up down shell configure build run rebuild logs configure-local build-local deps-ubuntu

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
	$(COMPOSE) exec $(SERVICE) sh -lc "cmake -E rm -rf $(BUILD_DIR) && cmake -S . -B $(BUILD_DIR) && cmake --build $(BUILD_DIR) && ./$(BUILD_DIR)/machine"

logs:
	$(COMPOSE) logs -f $(SERVICE)

configure-local:
	cmake -S . -B $(LOCAL_BUILD_DIR)

build-local:
	cmake --build $(LOCAL_BUILD_DIR)

deps-ubuntu:
	sudo apt update
	sudo apt install -y build-essential cmake libsqlite3-dev libcurl4-openssl-dev

deps-windows:
	choco install -y cmake ninja
	test -d "$(VCPKG_ROOT)" || git clone https://github.com/microsoft/vcpkg "$(VCPKG_ROOT)"
	cmd.exe /C "$(VCPKG_ROOT)/bootstrap-vcpkg.bat"
	"$(VCPKG_ROOT)/vcpkg.exe" install sqlite3 curl
