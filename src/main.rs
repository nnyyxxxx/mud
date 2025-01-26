mod constants;
mod game;
mod player;
mod world;

use game::{display_world, process_turn};
use world::World;

fn main() {
    let mut world = World::new();
    display_world(&world);

    while process_turn(&mut world) {
        display_world(&world);
    }
}
