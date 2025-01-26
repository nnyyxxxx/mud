use crate::{constants::*, player::Player, world::World};
use std::io::{self, Write};

pub fn display_world(world: &World) {
    print!("{}", CLEAR_SCREEN);
    display_map(world);
    println!();
    display_stats(&world.players);
    println!("Player {}'s turn!\n", world.current_player + 1);
}

fn display_map(world: &World) {
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let tile = draw_tile(world.map[y][x], x, y, &world.players);
            print!("{} ", tile);
        }
        println!();
    }
}

fn display_stats(players: &[Player]) {
    for (i, player) in players.iter().enumerate() {
        println!(
            "Player {} - Health: {} Gold: {}\n",
            i + 1,
            player.health,
            player.gold
        );
    }
}

fn draw_tile(tile: char, x: usize, y: usize, players: &[Player]) -> char {
    players
        .iter()
        .enumerate()
        .find(|(_, p)| p.x == x && p.y == y)
        .map_or(tile, |(i, _)| PLAYER_CHARS[i])
}

pub fn process_turn(world: &mut World) -> bool {
    print!("{}", MSG_INPUT_PROMPT);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let should_continue = match input.trim() {
        "w" | "a" | "s" | "d" => handle_move(world, input.trim().chars().next().unwrap()),
        "f" => handle_attack(world),
        "q" => false,
        _ => true,
    };

    should_continue
}

pub fn check_win_condition(world: &World, player: &Player) -> bool {
    if player.gold >= GOLD_TO_WIN {
        println!(
            "Player {} wins by collecting all gold!",
            world.current_player + 1
        );
        return true;
    }
    false
}

fn try_collect_gold(world: &mut World) {
    let player = &mut world.players[world.current_player];
    if world.map[player.y][player.x] != GOLD_TILE {
        return;
    }

    player.gold += GOLD_VALUE;
    world.map[player.y][player.x] = EMPTY_TILE;
}

fn next_turn(world: &mut World) {
    world.current_player = (world.current_player + 1) % PLAYER_COUNT;
}

fn handle_move(world: &mut World, direction: char) -> bool {
    let moved = world.players[world.current_player].handle_movement(direction);
    if !moved {
        return true;
    }

    try_collect_gold(world);
    display_world(world);

    if check_win_condition(world, &world.players[world.current_player]) {
        return false;
    }

    next_turn(world);
    true
}

fn check_combat_win(world: &World, target: usize) -> bool {
    if world.players[target].health <= 0 {
        println!("Player {} wins!", world.current_player + 1);
        return true;
    }
    false
}

fn perform_attack(world: &mut World, target: usize) -> bool {
    world.players[target].health -= ATTACK_DAMAGE;
    check_combat_win(world, target)
}

fn handle_attack(world: &mut World) -> bool {
    let target = (world.current_player + 1) % PLAYER_COUNT;
    let (px, py) = {
        let player = &world.players[world.current_player];
        (player.x, player.y)
    };
    let target_player = &world.players[target];

    if !World::is_adjacent(px, py, target_player.x, target_player.y) {
        return true;
    }

    if perform_attack(world, target) {
        return false;
    }

    next_turn(world);
    true
}
