# Composition vs Inheritance

A Rust educational project demonstrating how composition with traits differs from traditional OOP inheritance patterns.

## Overview

This project provides a side-by-side comparison of:
- **Traditional OOP**: Class-based inheritance hierarchies (Java/C# style)
- **Rust's Approach**: Trait-based composition with component patterns

Using a game entity system as an example, this code illustrates why "composition over inheritance" is a core principle in Rust and how it provides greater flexibility than rigid class hierarchies.

## What's Demonstrated

### Concepts Covered

- **Traits as Capabilities**: `Position`, `Movable`, `Health`, `Combatant`, `Updatable`, `Drawable`
- **Component Pattern**: Reusable building blocks (`PositionComponent`, `HealthComponent`, etc.)
- **Flexible Composition**: Entities built from components rather than inheriting from base classes
- **Runtime Capability Addition**: Adding/removing features dynamically (e.g., granting flight via `Option<FlyingComponent>`)
- **Trait Objects**: Polymorphism through `dyn Entity` for heterogeneous collections
- **Generic Functions**: Writing code that works with any type implementing required traits

### Example Entities

- **Enemy**: Basic ground enemy with position, health, and combat
- **FlyingEnemy**: Enemy with all basic features plus flight capability
- **Player**: Player character with optional flight ability that can be granted at runtime

## Running the Project

### Prerequisites

- Rust toolchain (1.56.0 or later)

### Build and Run

```bash
cargo build
cargo run
```

## Project Structure

```
composition-vs-inheritance/
├── src/
│   └── main.rs          # Complete demonstration with examples
├── Cargo.toml           # Project manifest
├── LICENSE              # Public Domain dedication (UNLICENSE)
└── README.md            # This file
```

## Key Takeaways

- Rust uses traits instead of inheritance hierarchies
- Components are composed into entities, not inherited
- Capabilities can be added/removed at runtime
- Traits enable polymorphism without rigid hierarchies
- Generic functions work with any type implementing required traits
- More flexible: easy to add flying to players, swimming to enemies, etc.

## Code Highlights

### The Problem with Inheritance

In traditional OOP, adding cross-cutting concerns (like flight) requires complex hierarchy changes:

```
GameObject
├── Enemy
│   └── FlyingEnemy
└── Player
    └── FlyingPlayer (?)
```

What if you want swimming? Invisible? Multiple combinations become unwieldy.

### The Composition Solution

With traits and components, you mix and match capabilities:

```rust
struct Player {
    position: PositionComponent,
    health: HealthComponent,
    combat: CombatComponent,
    flying: Option<FlyingComponent>,  // Optional!
}
```

Any entity can have any combination of components. No rigid hierarchy required.

## Learning Resources

- [Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust by Example - Traits](https://doc.rust-lang.org/rust-by-example/trait.html)
- [Composition over Inheritance](https://en.wikipedia.org/wiki/Composition_over_inheritance)
- [Related article on Medium](https://medium.com/rustaceans/composition-over-inheritance-93300c33918a)

## License

This project is dedicated to the Public Domain. See LICENSE file for details.
