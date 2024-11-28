#!/bin/python3

import os
import shutil
import platform
import subprocess


def log(message):
    """Log messages to the console."""
    print(f"[INFO] {message}")


def remove_game_folders():
    """Remove existing game folders."""
    folders = ["minecraft-rust-client-1", "minecraft-rust-client-2", "minecraft-rust-server"]
    for folder in folders:
        if os.path.exists(folder):
            log(f"Removing folder: {folder}")
            shutil.rmtree(folder)
        else:
            log(f"Folder does not exist: {folder}")


def create_game_folders():
    """Create necessary game folders for clients and server."""
    folders = [
        "minecraft-rust-client-1/bin", "minecraft-rust-client-1/saves",
        "minecraft-rust-client-2/bin", "minecraft-rust-client-2/saves",
        "minecraft-rust-server/bin", "minecraft-rust-server/saves"
    ]

    for folder in folders:
        os.makedirs(folder, exist_ok=True)
        log(f"Created folder: {folder}")

    # Copy the `data` folder into each game folder
    for target in ["minecraft-rust-client-1", "minecraft-rust-client-2", "minecraft-rust-server"]:
        if os.path.exists("data"):
            log(f"Copying 'data' to {target}/data")
            shutil.copytree("data", os.path.join(target, "data"), dirs_exist_ok=True)

    # Create empty `servers.ron` files
    for target in ["minecraft-rust-client-1", "minecraft-rust-client-2"]:
        servers_ron_path = os.path.join(target, "servers.ron")
        open(servers_ron_path, "w").close()
        log(f"Created empty file: {servers_ron_path}")


def copy_binaries(build_type="debug"):
    """
    Copy compiled binaries into the appropriate folders.
    :param build_type: Either 'debug' or 'release'.
    """
    if build_type not in ["debug", "release"]:
        raise ValueError("build_type must be 'debug' or 'release'")

    system = platform.system()

    if system == "Windows":
        binary_extension = ".exe"
    else:
        binary_extension = ""

    binaries = [
        (f"target/{build_type}/client{binary_extension}", f"minecraft-rust-client-1/bin/minecraft-rust{binary_extension}"),
        (f"target/{build_type}/client{binary_extension}", f"minecraft-rust-client-2/bin/minecraft-rust{binary_extension}"),
        (f"target/{build_type}/server{binary_extension}", f"minecraft-rust-server/bin/minecraft-rust-server{binary_extension}"),
    ]

    for src, dest in binaries:
        if os.path.exists(src):
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            shutil.copy(src, dest)
            log(f"Copied {src} to {dest}")
        else:
            log(f"Source binary does not exist: {src}")


def build_cargo(build_type="debug"):
    """
    Run the cargo build command.
    :param build_type: Either 'debug' or 'release'.
    """
    log(f"Running cargo build ({build_type})...")
    command = ["cargo", "build"]
    if build_type == "release":
        command.append("--release")

    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.returncode == 0:
        log("Cargo build completed successfully.")
    else:
        log(f"Cargo build failed: {result.stderr}")
        raise RuntimeError("Cargo build failed")


def main():
    import argparse

    parser = argparse.ArgumentParser(description="Build script for minecraft-rust.")
    parser.add_argument("command", choices=["debug", "release", "remove-game-folders"],
                        help="Command to execute: 'debug', 'release', or 'remove-game-folders'.")
    args = parser.parse_args()

    if args.command == "remove-game-folders":
        log("Cleaning game folders...")
        remove_game_folders()
    elif args.command in ["debug", "release"]:
        log(f"Starting build process ({args.command})...")
        remove_game_folders()
        create_game_folders()
        build_cargo(build_type=args.command)
        copy_binaries(build_type=args.command)
        log(f"Build process ({args.command}) completed successfully.")
    else:
        log("Invalid command. Use 'debug', 'release', or 'clean'.")


if __name__ == "__main__":
    main()
