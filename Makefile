APP_NAME = nano-vault
IMAGE_NAME = ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest

.PHONY: all build docker-build emulate clean git-init

all: build

docker-build:
	@echo "Building Nano Vault binary for Ledger Nano X using Ledger Dev Tools Docker container..."
	docker run --rm -v $(CURDIR):/app -w /app $(IMAGE_NAME) cargo ledger build nanox -- -Z build-std=core,alloc

emulate:
	@echo "Starting Speculos Ledger Nano X emulator for Nano Vault..."
	docker run --rm -it -p 5000:5000 -v $(CURDIR):/app $(IMAGE_NAME) speculos /app/target/thumbv6m-none-eabi/release/$(APP_NAME) --model nanox

git-init:
	@git init
	@git add .
	@git commit -m "initial commit"
	@echo "Git repository successfully initialized with initial commit!"

clean:
	rm -rf target
