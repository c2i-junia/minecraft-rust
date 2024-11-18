# OS detection
ifeq ($(OS),Windows_NT)
    RMDIR = rmdir /S /Q
    COPY_CLIENT_DEBUG = copy target\debug\client.exe minecraft-rust\bin\minecraft-rust.exe
    COPY_SERVER_DEBUG = copy target\debug\server.exe minecraft-rust\bin\minecraft-rust-server.exe
    COPY_CLIENT_RELEASE = copy target\release\client.exe minecraft-rust\bin\minecraft-rust.exe
    COPY_SERVER_RELEASE = copy target\release\server.exe minecraft-rust\bin\minecraft-rust-server.exe
else
    RMDIR = rm -rf
    COPY_CLIENT_DEBUG = cp target/debug/client minecraft-rust/bin/minecraft-rust
    COPY_SERVER_DEBUG = cp target/debug/server minecraft-rust/bin/minecraft-rust-server
    COPY_CLIENT_RELEASE = cp target/release/client minecraft-rust/bin/minecraft-rust
    COPY_SERVER_RELEASE = cp target/release/server minecraft-rust/bin/minecraft-rust-server
endif

debug:
	cargo build
	$(MAKE) prepare
	$(COPY_CLIENT_DEBUG)
	$(COPY_SERVER_DEBUG)

release:
	cargo build --release
	$(MAKE) prepare
	$(COPY_CLIENT_RELEASE)
	$(COPY_SERVER_RELEASE)

check: 
	cargo check

fmt: 
	cargo fmt

remove-game-folder:
	$(RMDIR) minecraft-rust

# Internal commands
prepare:
ifeq ($(OS),Windows_NT)
	if not exist minecraft-rust mkdir minecraft-rust
	if not exist minecraft-rust\bin mkdir minecraft-rust\bin
	if not exist minecraft-rust\saves mkdir minecraft-rust\saves
	xcopy /E /I /Q /Y data minecraft-rust\data
	type nul > minecraft-rust\servers.ron
else
	mkdir -p minecraft-rust minecraft-rust/bin minecraft-rust/saves
	cp -r data minecraft-rust/
	touch minecraft-rust/servers.ron
endif
