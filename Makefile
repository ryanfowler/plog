.PHONY: all
all:
	@echo "plog"
	@echo "make <cmd>"
	@echo ""
	@echo "commands:"
	@echo "  build       - build the plog docker container"
	@echo "  lint        - run clippy in a docker container"
	@echo "  test        - run tests in a docker container"

.PHONY: build
build:
	@docker build -t plog .

lint: build
	@docker run -i --rm --name plog plog cargo clippy

test: build
	@docker run -i --rm --name plog plog cargo test
