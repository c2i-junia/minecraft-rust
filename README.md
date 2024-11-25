# minecraft-rust

Minecraft game clone written in Rust, using the Bevy game engine.

<details open>
  <summary>game preview :</summary>
  <img src="https://github.com/eztaah/minecraft-rust/blob/main/docs/gameplay.gif" alt="Aperçu du projet" width="700">
</details>

<br>

## Table of contents

- [Controls](#controls)

- [Building from source](#building-from-source)

- [Contributing](#contributing)

<br>

## Controls

#### Movement
| Action             | Key               |
|--------------------|-------------------|
| Jump               | `Space`           |
| Strafe Left        | `A` / `Arrow Left`|
| Strafe Right       | `D` / `Arrow Right` |
| Walk Backward      | `S` / `Arrow Down` |
| Walk Forward       | `W` / `Arrow Up`  |
| Toggle Fly Mode    | `F`               |
| Fly Up             | `Space`           |
| Fly Down           | `Left Shift`      |

#### Gameplay
| Action             | Key               |
|--------------------|-------------------|
| Destroy Block      | `Left Mouse Button` |
| Place Block        | `Right Mouse Button` |

#### Inventory
| Action                        | Key/Mouse                        |
|-------------------------------|-----------------------------------|
| Open/Close Inventory          | `E`                              |
| Pick up stack                 | `Left Click`                     |
| Pick up half of stack         | `Right Click` (with empty mouse) |
| Deposit 1 item                | `Right Click` (over valid stack) |
| Deposit MAX items             | `Left Click` (over valid stack)  |
| Exchange stacks               | `Left Click` (over a different stack or full valid stack) |

> *A "valid stack" refers to a stack in the inventory that is either empty or contains the same items as the mouse cursor.*

#### Miscellaneous
| Action                        | Key               |
|-------------------------------|-------------------|
| Toggle FPS Display            | `F3`             |
| Toggle Perspective            | `F5`             |
| Toggle Chunk Debug            | `F4`             |
| Toggle Block Debug            | `F6`             |
| Decrease Render Distance      | `O`              |
| Increase Render Distance      | `P`              |
| Exit Game                     | `Escape`         |

<br>



## Building from source

### Prerequisites

To build and run this project, you need the following tools and dependencies installed:

#### 1. **Rust**
- Install Rust using [Rustup](https://rustup.rs)
- After installation, add the **Nightly toolchain** with the Cranelift backend:
  ```bash
  rustup install nightly
  rustup default nightly
  rustup component add rustc-codegen-cranelift-preview --toolchain nightly
  ```

#### 2. **Python**
- `Python 3.7+` is required to use the provided build script for building the project.


#### 3. **Dependencies**

Install the required dependencies based on your operating system:

##### Arch Linux
```bash
sudo pacman -S base-devel mold clang vulkan-radeon vulkan-tools
```
- Replace `vulkan-radeon` with:   
  - `vulkan-intel` for Intel GPUs.   
  - `nvidia-utils` for NVIDIA GPUs.   

##### Ubuntu/Debian
```bash
sudo apt update && sudo apt install -y \
    build-essential mold clang mesa-vulkan-drivers vulkan-tools
```
- For NVIDIA GPUs, also install:
    ```shell
    sudo apt install -y nvidia-driver nvidia-vulkan-icd
    ```

### Running the Project

To compile and run the game locally, use the following commands:

Note: the first compilation will be slow depending on your hardware, next compilations will be incremental and thus faster.

```sh
# Clone the repository
git clone https://github.com/c2i-junia/minecraft-rust

# Navigate to the project directory
cd minecraft-rust
```

Debug mode:
```sh
./run1.sh
```

Release mode:
```sh
# Build the project in debug or release mode
./build.py release

# Run the executable
./minecraft-rust-client-1/bin/minecraft-rust
```

<br>

## Contributing

Feel free to submit issues or open pull requests. If you want to know where to help, refer to the existing issues.

### Guidelines 

#### Format
Run `cargo fmt` before committing.

#### Commit messages:

We follow the [Conventional Commit specification](https://www.conventionalcommits.org/en/v1.0.0/).
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Our commit types are inspired by the [Karma specification](http://karma-runner.github.io/6.4/dev/git-commit-msg.html)

Allowed <type> values: 
- **feat** for a new feature for the user, not a new feature for build script. Such commit will trigger a release bumping a MINOR version.
- **fix** for a bug fix for the user, not a fix to a build script. Such commit will trigger a release bumping a PATCH version.
- **perf** for performance improvements. Such commit will trigger a release bumping a PATCH version.
- **docs** for changes to the documentation.
- **style** for formatting changes, missing semicolons, etc.
- **refactor** for refactoring production code, e.g. renaming a variable.
- **test** for adding missing tests, refactoring tests; no production code change.
- **build** for updating build configuration, development tools or other changes irrelevant to the user.

Write commit messages in the present tense (e.g., "Add feature X" instead of "Added feature X").

If a commit is co-authored by multiple people, do not hesitate to add a `Co-authored-by` field. See [GitHub documentation](https://docs.github.com/en/pull-requests/committing-changes-to-your-project/creating-and-editing-commits/creating-a-commit-with-multiple-authors). For example: 
```sh
$ git commit -m "Refactor usability tests.
>
>
Co-authored-by: NAME <NAME@EXAMPLE.COM>
Co-authored-by: ANOTHER-NAME <ANOTHER-NAME@EXAMPLE.COM>"
```

#### Branches
- Use the naming convention `<type>/<name>` for branches introducing new features. Only use lowercase letters, numbers, and dashes.
- The `main` branch should always compile successfully and be free of warnings. Use `cargo check`.
- Experimental branches are allowed to include code that does not build successfully.
- Prefer rebasing over merging.
