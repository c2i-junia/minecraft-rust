# minecraft-rust

Minecraft game clone written in Rust, using the Bevy game engine.

<details open>
  <summary>game preview :</summary>
  <img src="https://github.com/eztaah/minecraft-rust/blob/main/docs/gameplay.gif" alt="Aperçu du projet" width="500">
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

To run this project, you need to have the following installed:

- **Rust** (Stable toolchain) (This can be installed in several ways depending on your setup, the most standard way is to use rustup.rs.)
- **Make**


### Running the Project

To compile and run the game locally, use the following commands:

Note: the first compilation will be slow depending on your hardware, next compilations will be incremental and thus faster.

```sh
# Clone the repository
git clone https://github.com/c2i-junia/minecraft-rust

# Navigate to the project directory
cd minecraft-rust

# Build the project in debug or release mode
make debug    # or `make release`

# Run the executable
./minecraft-rust/bin/minecraft-rust
```

<br>

## Contributing

Feel free to submit issues or open pull requests. If you want to know where to help, refer to the existing issues.

### Guidelines 

1. Run `cargo fmt` before committing.
2. **Commit Messages**:
   - Use a capitalized first letter.
   - Write commit messages in the present tense (e.g., "Add feature X" instead of "Added feature X").
3. **Branches**:
   - Use the naming convention `feature/<description>` for branches introducing new features.
   - The `main` branch should always compile successfully and be free of warnings.
   - Experimental branches are allowed to include code that does not build successfully.
   - Prefer rebasing over merging.
