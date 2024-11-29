remove-game-folders:
    rm -rf minecraft-rust-client-1 minecraft-rust-client-2 minecraft-rust-server

create-game-folders:
    # client 1
    mkdir -p minecraft-rust-client-1 minecraft-rust-client-1/saves
    cp -ru data minecraft-rust-client-1/
    touch minecraft-rust-client-1/servers.ron
    rm -rf ./minecraft-rust-client-1/bin

    # client 2
    mkdir -p minecraft-rust-client-2 minecraft-rust-client-2/saves
    cp -ru data minecraft-rust-client-2/
    touch minecraft-rust-client-2/servers.ron
    rm -rf ./minecraft-rust-client-2/bin

    # server
    mkdir -p minecraft-rust-server minecraft-rust-server/saves
    rm -rf ./minecraft-rust-server/bin

debug: create-game-folders
    cargo build --features=bevy/dynamic_linking

release: remove-game-folders create-game-folders
    cargo build --release
    
    mkdir -p minecraft-rust-client-1/bin minecraft-rust-client-2/bin minecraft-rust-server/bin
    cp target/release/client minecraft-rust-client-1/bin/minecraft-rust
    cp target/release/client minecraft-rust-client-2/bin/minecraft-rust
    cp target/release/server minecraft-rust-server/bin/minecraft-rust-server
