mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
current_dir := $(dir $(mkfile_path))

TARGET = x86_64-unknown-linux-gnu

build-image:
	docker build -t rust-cross-compilation .

setup-target:
	rustup target add $(TARGET)

prepare-dirs:
	mkdir -p $(current_dir)/output

build: setup-target build-image prepare-dirs
	$(eval CONTAINER_ID := $(shell docker run -d -v $(current_dir):/usr/src/config-injector rust-cross-compilation cargo build --release --target $(TARGET)))
	@echo "Container ID: $(CONTAINER_ID)"
	# Wait for container to finish and show logs
	@docker logs -f $(CONTAINER_ID)

copy-binary: build
	docker cp $(CONTAINER_ID):/usr/src/config-injector/target/$(TARGET)/release/config-injector $(current_dir)/output/config-injector
	docker stop $(CONTAINER_ID)
	docker rm $(CONTAINER_ID)

all: build copy-binary

clean:
	rm -rf ./target
	rm -rf ./output
