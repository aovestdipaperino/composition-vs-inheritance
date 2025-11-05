// Composition vs Inheritance: A Deep Dive
// This file demonstrates how Rust's approach to composition differs from
// traditional OOP inheritance patterns in Java/C#

// =============================================================================
// PART 1: The Traditional OOP Inheritance Approach (conceptually)
// =============================================================================

/*
In Java/C#, you might model a game entity system like this:

abstract class GameObject {
    protected String name;
    protected int x, y;

    public abstract void update();
    public void move(int dx, int dy) {
        x += dx;
        y += dy;
    }
}

class Enemy extends GameObject {
    private int health;
    private int damage;

    public void update() {
        // AI logic
    }

    public void attack(Player player) {
        player.takeDamage(damage);
    }
}

class FlyingEnemy extends Enemy {
    private int altitude;

    @Override
    public void move(int dx, int dy) {
        // Flying enemies move differently
        super.move(dx, dy);
        altitude += dy;
    }
}

// Problem: What if we want a Flying Player? Or a Swimming Enemy?
// The inheritance hierarchy becomes rigid and doesn't handle cross-cutting concerns well.
*/

// =============================================================================
// PART 2: Rust's Trait-Based Composition Approach
// =============================================================================

// Define capabilities as traits instead of base classes
trait Position {
    fn get_position(&self) -> (f32, f32);
    fn set_position(&mut self, x: f32, y: f32);
}

trait Movable: Position {
    fn move_by(&mut self, dx: f32, dy: f32) {
        let (x, y) = self.get_position();
        self.set_position(x + dx, y + dy);
    }
}

trait Health {
    fn get_health(&self) -> i32;
    fn set_health(&mut self, health: i32);
    fn is_alive(&self) -> bool {
        self.get_health() > 0
    }
    fn take_damage(&mut self, damage: i32) {
        let new_health = self.get_health() - damage;
        self.set_health(new_health.max(0));
    }
}

trait Combatant: Health {
    fn get_attack_damage(&self) -> i32;
    fn attack<T: Health>(&self, target: &mut T) {
        target.take_damage(self.get_attack_damage());
    }
}

trait Updatable {
    fn update(&mut self, delta_time: f32);
}

trait Drawable {
    fn draw(&self) -> String;
}

// =============================================================================
// PART 3: Component Structs (Composition Building Blocks)
// =============================================================================

#[derive(Debug, Clone)]
struct PositionComponent {
    x: f32,
    y: f32,
}

impl PositionComponent {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct HealthComponent {
    current: i32,
    max: i32,
}

impl HealthComponent {
    fn new(max: i32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Debug, Clone)]
struct CombatComponent {
    damage: i32,
}

impl CombatComponent {
    fn new(damage: i32) -> Self {
        Self { damage }
    }
}

#[derive(Debug, Clone)]
struct FlyingComponent {
    altitude: f32,
    max_altitude: f32,
}

impl FlyingComponent {
    fn new(max_altitude: f32) -> Self {
        Self {
            altitude: 0.0,
            max_altitude,
        }
    }
}

// =============================================================================
// PART 4: Concrete Entities Using Composition
// =============================================================================

// A basic enemy composed of multiple components
#[derive(Debug)]
struct Enemy {
    name: String,
    position: PositionComponent,
    health: HealthComponent,
    combat: CombatComponent,
}

impl Enemy {
    fn new(name: impl Into<String>, x: f32, y: f32, health: i32, damage: i32) -> Self {
        Self {
            name: name.into(),
            position: PositionComponent::new(x, y),
            health: HealthComponent::new(health),
            combat: CombatComponent::new(damage),
        }
    }
}

// Implement traits for Enemy by delegating to components
impl Position for Enemy {
    fn get_position(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }
}

impl Movable for Enemy {}

impl Health for Enemy {
    fn get_health(&self) -> i32 {
        self.health.current
    }

    fn set_health(&mut self, health: i32) {
        self.health.current = health.min(self.health.max);
    }
}

impl Combatant for Enemy {
    fn get_attack_damage(&self) -> i32 {
        self.combat.damage
    }
}

impl Updatable for Enemy {
    fn update(&mut self, delta_time: f32) {
        // Simple AI: move right slowly
        self.move_by(10.0 * delta_time, 0.0);
    }
}

impl Drawable for Enemy {
    fn draw(&self) -> String {
        format!("👾 {} at ({:.1}, {:.1}) HP: {}",
            self.name, self.position.x, self.position.y, self.health.current)
    }
}

// A flying enemy that composes the same components plus a flying component
#[derive(Debug)]
struct FlyingEnemy {
    name: String,
    position: PositionComponent,
    health: HealthComponent,
    combat: CombatComponent,
    flying: FlyingComponent,
}

impl FlyingEnemy {
    fn new(name: impl Into<String>, x: f32, y: f32, health: i32, damage: i32) -> Self {
        Self {
            name: name.into(),
            position: PositionComponent::new(x, y),
            health: HealthComponent::new(health),
            combat: CombatComponent::new(damage),
            flying: FlyingComponent::new(100.0),
        }
    }

    fn fly_up(&mut self, amount: f32) {
        self.flying.altitude = (self.flying.altitude + amount).min(self.flying.max_altitude);
    }

    fn fly_down(&mut self, amount: f32) {
        self.flying.altitude = (self.flying.altitude - amount).max(0.0);
    }
}

impl Position for FlyingEnemy {
    fn get_position(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }
}

impl Movable for FlyingEnemy {}

impl Health for FlyingEnemy {
    fn get_health(&self) -> i32 {
        self.health.current
    }

    fn set_health(&mut self, health: i32) {
        self.health.current = health.min(self.health.max);
    }
}

impl Combatant for FlyingEnemy {
    fn get_attack_damage(&self) -> i32 {
        self.combat.damage
    }
}

impl Updatable for FlyingEnemy {
    fn update(&mut self, delta_time: f32) {
        // Flying enemies have different behavior
        self.move_by(15.0 * delta_time, 5.0 * delta_time);

        // Bob up and down
        if self.flying.altitude > 50.0 {
            self.fly_down(20.0 * delta_time);
        } else {
            self.fly_up(20.0 * delta_time);
        }
    }
}

impl Drawable for FlyingEnemy {
    fn draw(&self) -> String {
        format!("🦅 {} at ({:.1}, {:.1}) altitude: {:.1} HP: {}",
            self.name, self.position.x, self.position.y,
            self.flying.altitude, self.health.current)
    }
}

// A player that can also fly (easy to add with composition!)
#[derive(Debug)]
struct Player {
    name: String,
    position: PositionComponent,
    health: HealthComponent,
    combat: CombatComponent,
    flying: Option<FlyingComponent>, // Optional flying capability!
}

impl Player {
    fn new(name: impl Into<String>, x: f32, y: f32) -> Self {
        Self {
            name: name.into(),
            position: PositionComponent::new(x, y),
            health: HealthComponent::new(100),
            combat: CombatComponent::new(25),
            flying: None,
        }
    }

    fn grant_flight(&mut self, max_altitude: f32) {
        self.flying = Some(FlyingComponent::new(max_altitude));
    }

    fn can_fly(&self) -> bool {
        self.flying.is_some()
    }
}

impl Position for Player {
    fn get_position(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }
}

impl Movable for Player {}

impl Health for Player {
    fn get_health(&self) -> i32 {
        self.health.current
    }

    fn set_health(&mut self, health: i32) {
        self.health.current = health.min(self.health.max);
    }
}

impl Combatant for Player {
    fn get_attack_damage(&self) -> i32 {
        self.combat.damage
    }
}

impl Drawable for Player {
    fn draw(&self) -> String {
        let flight_status = if let Some(ref flying) = self.flying {
            format!(" [Flying at {:.1}]", flying.altitude)
        } else {
            String::new()
        };

        format!("🧙 {} at ({:.1}, {:.1}) HP: {}{}",
            self.name, self.position.x, self.position.y,
            self.health.current, flight_status)
    }
}

// =============================================================================
// PART 5: Polymorphism Through Trait Objects
// =============================================================================

// Create a combined trait for entities in the game
trait Entity: Updatable + Drawable {}

// Blanket implementation: anything that is Updatable + Drawable is an Entity
impl<T: Updatable + Drawable> Entity for T {}

// We can store different types that implement the same traits
fn simulate_game_loop() {
    println!("\n=== Game Simulation ===\n");

    let mut entities: Vec<Box<dyn Entity>> = vec![
        Box::new(Enemy::new("Goblin", 0.0, 0.0, 50, 10)),
        Box::new(FlyingEnemy::new("Dragon", 100.0, 50.0, 150, 30)),
    ];

    // Simulate 3 frames
    for frame in 1..=3 {
        println!("Frame {}:", frame);
        for entity in entities.iter_mut() {
            entity.update(0.1); // 0.1 second delta
            println!("  {}", entity.draw());
        }
        println!();
    }
}

// =============================================================================
// PART 6: Demonstration of Flexibility
// =============================================================================

fn demonstrate_combat() {
    println!("\n=== Combat System Demo ===\n");

    let mut player = Player::new("Alice", 0.0, 0.0);
    let mut enemy = Enemy::new("Orc", 10.0, 0.0, 30, 15);

    println!("{}", player.draw());
    println!("{}", enemy.draw());
    println!();

    // Player attacks enemy
    println!("Player attacks enemy for {} damage!", player.get_attack_damage());
    player.attack(&mut enemy);
    println!("{}", enemy.draw());
    println!();

    // Enemy attacks back
    println!("Enemy attacks player for {} damage!", enemy.get_attack_damage());
    enemy.attack(&mut player);
    println!("{}", player.draw());
}

fn demonstrate_composition_flexibility() {
    println!("\n=== Composition Flexibility Demo ===\n");

    let mut player = Player::new("Bob", 0.0, 0.0);
    println!("Initial: {}", player.draw());
    println!("Can fly? {}", player.can_fly());
    println!();

    // Grant flying ability at runtime!
    println!("Found magic wings! Granting flight ability...");
    player.grant_flight(150.0);
    println!("Can fly? {}", player.can_fly());
    println!();

    // Now player can fly
    if let Some(ref mut flying) = player.flying {
        flying.altitude = 75.0;
    }
    println!("{}", player.draw());
}

// =============================================================================
// PART 7: Trait-based Generic Functions
// =============================================================================

// This function works with ANY type that implements Position and Movable
fn move_towards<T: Position + Movable>(entity: &mut T, target_x: f32, target_y: f32, speed: f32) {
    let (x, y) = entity.get_position();
    let dx = target_x - x;
    let dy = target_y - y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance > 0.0 {
        let move_x = (dx / distance) * speed;
        let move_y = (dy / distance) * speed;
        entity.move_by(move_x, move_y);
    }
}

fn demonstrate_generic_functions() {
    println!("\n=== Generic Functions Demo ===\n");

    let mut enemy = Enemy::new("Slime", 0.0, 0.0, 20, 5);
    let mut player = Player::new("Charlie", 100.0, 100.0);

    println!("Before movement:");
    println!("  {}", enemy.draw());
    println!("  {}", player.draw());
    println!();

    // Move enemy towards player
    let (px, py) = player.get_position();
    println!("Enemy moves towards player...");
    move_towards(&mut enemy, px, py, 25.0);

    // Move player away
    println!("Player moves away...");
    move_towards(&mut player, 150.0, 150.0, 30.0);

    println!("\nAfter movement:");
    println!("  {}", enemy.draw());
    println!("  {}", player.draw());
}

// =============================================================================
// Main Function
// =============================================================================

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║   Composition Over Inheritance: Rust vs OOP Demo     ║");
    println!("╚═══════════════════════════════════════════════════════╝");

    demonstrate_combat();
    simulate_game_loop();
    demonstrate_composition_flexibility();
    demonstrate_generic_functions();

    println!("\n=== Key Takeaways ===");
    println!("✓ Rust uses traits instead of inheritance hierarchies");
    println!("✓ Components are composed into entities, not inherited");
    println!("✓ Capabilities can be added/removed at runtime (Option<T>)");
    println!("✓ Traits enable polymorphism without rigid hierarchies");
    println!("✓ Generic functions work with any type implementing required traits");
    println!("✓ More flexible: easy to add flying to players, swimming to enemies, etc.");
}
