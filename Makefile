# OS detection
ifeq ($(OS),Windows_NT)
    RMDIR = rmdir /S /Q
    COPY_CLIENT_DEBUG = copy target\debug\client.exe minecraft-rust-client\bin\minecraft-rust.exe
    COPY_CLIENT_RELEASE = copy target\release\client.exe minecraft-rust-client\bin\minecraft-rust.exe
    COPY_SERVER_DEBUG = copy target\debug\server.exe minecraft-rust-server\bin\minecraft-rust-server.exe
    COPY_SERVER_RELEASE = copy target\release\server.exe minecraft-rust-server\bin\minecraft-rust-server.exe
else
    RMDIR = rm -rf
    COPY_CLIENT_DEBUG = cp target/debug/client minecraft-rust-client/bin/minecraft-rust
    COPY_CLIENT_RELEASE = cp target/release/client minecraft-rust-client/bin/minecraft-rust
    COPY_SERVER_DEBUG = cp target/debug/server minecraft-rust-server/bin/minecraft-rust-server
    COPY_SERVER_RELEASE = cp target/release/server minecraft-rust-server/bin/minecraft-rust-server
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
	$(RMDIR) minecraft-rust-client
	$(RMDIR) minecraft-rust-server

# Internal commands
prepare:
ifeq ($(OS),Windows_NT)
	if not exist minecraft-rust-client mkdir minecraft-rust-client
	if not exist minecraft-rust-client\bin mkdir minecraft-rust-client\bin
	if not exist minecraft-rust-client\saves mkdir minecraft-rust-client\saves
	if not exist minecraft-rust-server mkdir minecraft-rust-server
	if not exist minecraft-rust-server\bin mkdir minecraft-rust-server\bin
	if not exist minecraft-rust-server\saves mkdir minecraft-rust-server\saves
	xcopy /E /I /Q /Y data minecraft-rust-client\data
	xcopy /E /I /Q /Y data minecraft-rust-server\data
	type nul > minecraft-rust-client\servers.ron
else
	mkdir -p minecraft-rust-client minecraft-rust-client/bin minecraft-rust-client/saves
	mkdir -p minecraft-rust-server minecraft-rust-server/bin minecraft-rust-server/saves
	cp -r data minecraft-rust-client/
	cp -r data minecraft-rust-server/
	touch minecraft-rust-client/servers.ron
endif
