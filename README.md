# minecraft-rust

Copy of Minecraft written in Rust, using the Bevy game engine

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

## Dependencies

ArchLinux :

```sh
sudo pacman -Syu vulkan-radeon vulkan-tools
```
