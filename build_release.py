#!/bin/python3

import os
import shutil
import platform
import subprocess
import sys

def log(message):
    """Log messages to the console."""
    print(f"[INFO] {message}")

def run_command(command):
    """Run a shell command and ensure it succeeds."""
    log(f"Executing: {command}")
    result = subprocess.run(command, shell=True)
    if result.returncode != 0:
        log(f"Command failed: {command}")
        sys.exit(1)

def main():
    if len(sys.argv) != 2:
        print("Usage: python build_package.py <version>")
        sys.exit(1)

    version = sys.argv[1]
    build_dir = "minecraft-rust"
    versioned_dir = ""

    # Step 1: Run `make remove-game-folder`
    run_command("make remove-game-folder")

    # Step 2: Run `make release`
    run_command("make release")

    # Step 3: Copy CHANGELOG.txt and LICENSE.txt into the build directory
    changelog_path = os.path.join(build_dir, "CHANGELOG.txt")
    license_path = os.path.join(build_dir, "LICENSE.txt")
    shutil.copy("CHANGELOG.txt", changelog_path)
    log(f"Copied CHANGELOG.txt to {changelog_path}")
    shutil.copy("LICENSE.txt", license_path)
    log(f"Copied LICENSE.txt to {license_path}")

    # Step 4: Create a `version.txt` file
    version_file_path = os.path.join(build_dir, "version.txt")
    with open(version_file_path, "w") as version_file:
        version_file.write(version)
    log(f"Created version.txt with version: {version}")

    # Step 5: Detect operating system
    os_name = platform.system().lower()

    if os_name == "linux":
        versioned_dir = f"{build_dir}-{version}-linux-x86_64"
        shutil.move(build_dir, versioned_dir)
        log(f"Renamed {build_dir} to {versioned_dir}")
        tar_file = f"{versioned_dir}.tar.gz"
        shutil.make_archive(versioned_dir, 'gztar', root_dir=versioned_dir)
        log(f"Compressed {versioned_dir} into {tar_file}")
    elif os_name == "windows":
        versioned_dir = f"{build_dir}-{version}-windows-x86_64"
        shutil.move(build_dir, versioned_dir)
        log(f"Renamed {build_dir} to {versioned_dir}")
        zip_file = f"{versioned_dir}.zip"
        shutil.make_archive(versioned_dir, 'zip', root_dir=versioned_dir)
        log(f"Compressed {versioned_dir} into {zip_file}")
    else:
        log("Unsupported operating system. Only Linux and Windows are supported.")
        sys.exit(1)

    log("Build and packaging process completed successfully.")

if __name__ == "__main__":
    main()
