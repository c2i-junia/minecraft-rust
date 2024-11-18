# OS detection
ifeq ($(OS),Windows_NT)
    RMDIR = rmdir /S /Q
    COPY_CLIENT_DEBUG = copy target\debug\client.exe minecraft-rust-client-1\bin\minecraft-rust.exe
    COPY_CLIENT_RELEASE = copy target\release\client.exe minecraft-rust-client-1\bin\minecraft-rust.exe
    COPY_SERVER_DEBUG = copy target\debug\server.exe minecraft-rust-server\bin\minecraft-rust-server.exe
    COPY_SERVER_RELEASE = copy target\release\server.exe minecraft-rust-server\bin\minecraft-rust-server.exe
    COPY_CLIENT_DEBUG_2 = copy target\debug\client.exe minecraft-rust-client-2\bin\minecraft-rust.exe
    COPY_CLIENT_RELEASE_2 = copy target\release\client.exe minecraft-rust-client-2\bin\minecraft-rust.exe
else
    RMDIR = rm -rf
    COPY_CLIENT_DEBUG = cp target/debug/client minecraft-rust-client-1/bin/minecraft-rust
    COPY_CLIENT_RELEASE = cp target/release/client minecraft-rust-client-1/bin/minecraft-rust
    COPY_SERVER_DEBUG = cp target/debug/server minecraft-rust-server/bin/minecraft-rust-server
    COPY_SERVER_RELEASE = cp target/release/server minecraft-rust-server/bin/minecraft-rust-server
    COPY_CLIENT_DEBUG_2 = cp target/debug/client minecraft-rust-client-2/bin/minecraft-rust
    COPY_CLIENT_RELEASE_2 = cp target/release/client minecraft-rust-client-2/bin/minecraft-rust
endif

debug:
	cargo build
	$(MAKE) prepare
	$(COPY_CLIENT_DEBUG)
	$(COPY_CLIENT_DEBUG_2)
	$(COPY_SERVER_DEBUG)

release:
	cargo build --release
	$(MAKE) prepare
	$(COPY_CLIENT_RELEASE)
	$(COPY_CLIENT_RELEASE_2)
	$(COPY_SERVER_RELEASE)

check: 
	cargo check

fmt: 
	cargo fmt

remove-game-folder:
	$(RMDIR) minecraft-rust-client-1
	$(RMDIR) minecraft-rust-client-2
	$(RMDIR) minecraft-rust-server

# Internal commands
prepare:
ifeq ($(OS),Windows_NT)
	if not exist minecraft-rust-client-1 mkdir minecraft-rust-client-1
	if not exist minecraft-rust-client-1\bin mkdir minecraft-rust-client-1\bin
	if not exist minecraft-rust-client-1\saves mkdir minecraft-rust-client-1\saves
	if not exist minecraft-rust-client-2 mkdir minecraft-rust-client-2
	if not exist minecraft-rust-client-2\bin mkdir minecraft-rust-client-2\bin
	if not exist minecraft-rust-client-2\saves mkdir minecraft-rust-client-2\saves
	if not exist minecraft-rust-server mkdir minecraft-rust-server
	if not exist minecraft-rust-server\bin mkdir minecraft-rust-server\bin
	if not exist minecraft-rust-server\saves mkdir minecraft-rust-server\saves
	xcopy /E /I /Q /Y data minecraft-rust-client-1\data
	xcopy /E /I /Q /Y data minecraft-rust-client-2\data
	xcopy /E /I /Q /Y data minecraft-rust-server\data
	type nul > minecraft-rust-client-1\servers.ron
	type nul > minecraft-rust-client-2\servers.ron
else
	mkdir -p minecraft-rust-client-1 minecraft-rust-client-1/bin minecraft-rust-client-1/saves
	mkdir -p minecraft-rust-client-2 minecraft-rust-client-2/bin minecraft-rust-client-2/saves
	mkdir -p minecraft-rust-server minecraft-rust-server/bin minecraft-rust-server/saves
	cp -r data minecraft-rust-client-1/
	cp -r data minecraft-rust-client-2/
	cp -r data minecraft-rust-server/
	touch minecraft-rust-client-1/servers.ron
	touch minecraft-rust-client-2/servers.ron
endif
