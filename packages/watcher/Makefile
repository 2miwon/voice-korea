ENV ?= local
SERVICE ?= watcher
WORKSPACE_ROOT ?= ../..
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)

BASE_DOMAIN ?= voice-korea.com
LOG_LEVEL ?= info

API_DOMAIN ?= https://voice-korea-api.$(ENV).$(BASE_DOMAIN)

ifeq ("$(ENV)","local")
	LOG_LEVEL ?= debug
	API_DOMAIN = http://0.0.0.0:3000
endif

BUILD_ENV ?= SERVICE=${SERVICE} ENV=$(ENV) VERSION=${VERSION} COMMIT=${COMMIT} API_DOMAIN=${API_DOMAIN}

setup.tool:
	cargo install cargo-lambda
run:
	$(BUILD_ENV) cargo lambda watch

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE)
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(WORKSPACE_ROOT)/target/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap

get-version:
	curl -X GET "http://localhost:9000/lambda-url/watcher/version"

execute-cron-job:
	curl -X POST "http://localhost:9000/lambda-url/watcher" -H "Content-Type: application/json" -d '{"source": "aws.events"}'
