# Slime Game 3D

A complex 3D physics-based slime game built in Rust using the Bevy game engine.

## Features

- **3D Slime Physics**: Play as a controllable slime blob with realistic physics
- **Dynamic Merging System**: Slimes automatically merge when they collide, combining their mass and color
- **Splitting Mechanics**: Large slimes can split into smaller ones when they exceed a certain mass
- **Enemy AI**: AI-controlled enemy slimes that wander around the arena and interact with the player
- **3D Environment**: Navigate through a 3D arena with obstacles, ramps, and walls
- **Dynamic Camera**: Smooth follow-camera that tracks the player slime
- **Physics-Based Gameplay**: Built with Bevy Rapier 3D physics engine

## Controls

- **WASD**: Move the slime horizontally
- **SPACE**: Jump/move upward
- **ESC**: Exit the game

## Game Mechanics

1. **Merging**: When slimes collide within a certain radius, they merge together. The resulting slime inherits a blended color from both slimes and gains their combined mass.

2. **Splitting**: When a slime grows too large (mass > 2.5), it automatically splits into two smaller slimes that bounce apart from each other.

3. **Color Mixing**: As slimes merge, their colors blend together, creating unique combinations.

4. **Scale**: The visual size of a slime is directly proportional to its mass (cube root relationship for realistic volume).

## Building and Running

### Prerequisites
- Rust 1.70 or later
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

## Dependencies

- **bevy**: Game engine with 3D rendering
- **bevy_rapier3d**: Physics engine with 3D rigid body dynamics
- **rand**: Random number generation for AI behavior
- **glam**: Vector math library

## Project Structure

- `src/main.rs`: Entry point and main application setup
- `src/slime.rs`: Slime entity system, movement, merging, and splitting logic
- `src/environment.rs`: Ground, walls, obstacles, and lighting setup
- `src/camera.rs`: Camera controller that follows the player slime
- `src/game_state.rs`: Game statistics and state management

## Performance Notes

The game uses dynamic physics simulation for all slimes. Performance may vary depending on:
- Number of active slimes
- Physics simulation accuracy settings
- Render quality

Enjoy the slimy adventures! 
