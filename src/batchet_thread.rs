extern crate core;

use std::collections::HashMap;
use std::{io, thread};
use std::borrow::BorrowMut;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread::Thread;


#[derive(PartialEq, Debug, Clone, Copy)]
enum Player {
    Stan,
    Ollie,
}

impl Player {
    fn other(&self) -> Player {
        match self {
            Player::Stan => Player::Ollie,
            Player::Ollie => Player::Stan
        }
    }
}


const DEBUG: bool = false;
const CACHE_SIZE: usize = 10_000;

/// moves must be in ascending order
fn batchet_turn(nb_stones: usize, moves: &Vec<usize>, curr_player: Player, depth: usize, cache: Arc<Mutex<HashMap<usize, (Player, Player)>>>) -> Player {
    if moves.contains(&nb_stones) {
        if DEBUG {
            print!("{}", "\t".repeat(depth));
            println!("Winner is {:?} because the nb_stones is: {}", curr_player, nb_stones);
        }
        return curr_player;
    }

    if nb_stones < moves[moves.len() - 2] { // less than the min of moves that is not 1
        let winner = if nb_stones % 2 == 0 { curr_player.other() } else { curr_player };
        if DEBUG {
            print!("{}", "\t".repeat(depth));
            println!("Winner is {:?} because the nb_stones is: {} and nb_stones % 2 is: {}", winner, nb_stones, nb_stones % 2);
        }
        return winner;
    }

    if let Some((first_player, winner)) = cache.lock().unwrap().get(&nb_stones) {
        let new_winner = if *first_player != curr_player { winner.other() } else { *winner };
        if DEBUG {
            print!("{}", "\t".repeat(depth));
            println!("[Reconnect] nb_stones: {}, curr_player: {:?} with other game with first_player: {:?} and winner: {:?}. Winner is: {:?}"
                     , nb_stones, curr_player, first_player, winner, new_winner
            );
        }
        return new_winner;
    }

    let valid_moves = moves.clone().into_iter().filter(|x| *x <= nb_stones).collect::<Vec<usize>>();

    for stone_move in valid_moves {
        if DEBUG {
            print!("{}", "\t".repeat(depth));
            println!("Player: {:?}, nb_stones: {}, move: {}", curr_player, nb_stones, stone_move);
        }
        let next_turn = batchet_turn(nb_stones - stone_move, moves, curr_player.other(), depth + 1, cache.clone());
        cache.lock().unwrap().insert(nb_stones - stone_move, (curr_player.other(), next_turn));
        if next_turn == curr_player {
            return curr_player;
        }
    }
    if DEBUG { println!(); }

    curr_player.other()
}

/// There are two players Stan and Ollie, who move alternately.
/// Stan always starts.
///
/// The legal moves consist in removing at least one but not more than `k` stones from the table.
/// The winner is the one to take the last stone.
///
/// Here we consider a variation of this game.
/// The number of stones that can be removed in a single move must be a member of a certain set of `m` numbers.
/// Among the `m` numbers there is always "1" and thus the game never stalls.
///
fn batchet(input: &str) {
    let (nb_stones, _rest) = input.split_once(' ').unwrap();
    let nb_stones = nb_stones.parse::<usize>().unwrap();
    // let nb_moves = rest.split_once(' ').unwrap().0.parse::<usize>().unwrap();

    let mut moves = input.split(' ').skip(2)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    moves.sort();
    moves.reverse();

    // always a 1 in moves, moves max length is 10

    let cache = Arc::new(Mutex::new(HashMap::new()));

    if nb_stones > CACHE_SIZE {
        (1..(nb_stones / CACHE_SIZE)).map(|i| {
            let cache_size = i * CACHE_SIZE;
            let moves = moves.clone();
            let cache = cache.clone();
            thread::spawn(move || {
                batchet_turn(cache_size, &moves, Player::Stan, 0, cache);
            })
        }).for_each(|t| t.join().unwrap())
    }
    let winner = batchet_turn(nb_stones, &moves, Player::Stan, 0, cache);
    println!("{:?} wins", winner)
}


fn main() {
    //let stdin = io::stdin();
    //for line in stdin.lock().lines().map(|l| l.unwrap()) {
    for line in "20 3 1 3 8\n21 3 1 3 8\n22 3 1 3 8\n23 3 1 3 8\n1000000 10 1 23 38 11 7 5 4 8 3 13\n999996 10 1 23 38 11 7 5 4 8 3 13\n".split('\n') {
        if line.trim().is_empty() { break; }
        batchet(line.trim());
    }
}

/*#[cfg(test)]
mod test {
    use super::batchet;

    #[test]
    fn test() {
        for line in "20 3 1 3 8\n21 3 1 3 8\n22 3 1 3 8\n23 3 1 3 8\n1000000 10 1 23 38 11 7 5 4 8 3 13\n999996 10 1 23 38 11 7 5 4 8 3 13\n".split('\n') {
            if line.trim().is_empty() { break; }
            batchet(line.trim());
        }
    }
}*/