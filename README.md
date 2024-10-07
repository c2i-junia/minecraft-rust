# minecraft-rust

A Minecraft game clone written in Rust, using the Bevy game engine.

<br>

## Controls

In world : 

| Action | Bind |
|----------------|----|
| Move Forward        | W / Arrow Up |
| Move Backward       | S / Arrow Down |
| Move Left           | A / Arrow Left |
| Move Right          | D / Arrow Right |
| Jump                | Space |
| Inventory           | E |
| Toggle Fly Mode     | F |
| Fly Up              | Space (only in Fly Mode) |
| Fly Down            | Left Shift (only in Fly Mode) |
| Toggle FPS Display  | F3 |
| Toggle View Mode    | F5 (First-Person / Third-Person) |
| Toggle Chunk Debug  | F4 |
| Toggle Block Debug  | F6 |
| Decrease Render Distance  | O |
| Increase Render Distance  | P |
| Save World  | L |
| Exit Game           | Escape |

In inventory :

| Action | Bind |
| --- | --- |
| Pick up stack | Left click |
| Pick up half of stack | Right click (with empty mouse) |
| Deposit 1 item | Right click (holding items, over valid stack) |
| Deposit MAX items | Left click (holding items, over valid stack) |
| Exchange stacks | Left click (holding items, over different stack / already full valid stack) |

> [!NOTE]
> In this context, "valid stack" means a stack in inventory, either empty or holding the same items as the mouse

<br>

## Getting Started

### Prerequisites

To run this project, you need to have the following installed:

- **Rust** (Stable toolchain)

This can be installed in several ways depending on your setup, the most standard way is to use rustup.rs.

### Running the Project

To compile and run the game locally, use the following commands:

Note: the first compilation will be slow depending on your hardware, next compilations will be incremental and thus faster.

```sh
# Clone the repository
git clone https://github.com/your-repo/minecraft-rust.git

# Navigate to the project directory
cd minecraft-rust

# Navigate to the client directory
cd client

# Start the client in dev mode
sh run.sh

# Alternative: start the client in release mode 
# Warning: slow compilation, but better runtime performance
cargo run --release
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
