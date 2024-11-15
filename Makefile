debug:
	cargo build
	mkdir -p minecraft-rust minecraft-rust/bin minecraft-rust/saves
	cp target/debug/client minecraft-rust/bin/minecraft-rust
	cp -r data minecraft-rust/
	touch minecraft-rust/servers.ron

release:
	cargo build --release
	mkdir -p minecraft-rust minecraft-rust/bin minecraft-rust/saves
	cp target/debug/client minecraft-rust/bin/minecraft-rust
	cp -r data minecraft-rust/
	touch minecraft-rust/servers.ron

check: 
	cargo check

remove-game-folder:
	rm -rf minecraft-rust
