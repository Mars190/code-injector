mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
current_dir := $(dir $(mkfile_path))

TARGET = x86_64-unknown-linux-gnu

build-image:
	docker build -t rust-cross-compilation .

prepare-dirs:
	mkdir -p $(current_dir)/output

build: build-image prepare-dirs
	$(eval CONTAINER_ID := $(shell docker run -d -v $(current_dir):/usr/src/config-injector rust-cross-compilation cargo build --release --target $(TARGET)))
	@docker logs -f $(CONTAINER_ID)
	docker cp $(CONTAINER_ID):/usr/src/config-injector/target/$(TARGET)/release/config-injector $(current_dir)/output/config-injector
	rm -rf ./target
	docker stop $(CONTAINER_ID)
	docker rm $(CONTAINER_ID)

clean:
	rm -rf ./target ./output