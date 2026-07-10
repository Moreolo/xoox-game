
use clap::Parser;

struct WinCon {
    sequence_length: usize,
    sequence_amount: usize,
}

enum GameState {
    WON1, WON2, DRAW, CONTINUE
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    sequence: String,
}

fn main() {
    let cli = Cli::parse();
    let sequence_length = cli.sequence.len();
    let player = (sequence_length % 2) as u8 ^ 1;

    let game_state = step(&cli.sequence, player);
    match game_state {
        GameState::WON1 => println!("Win for Player 1"),
        GameState::WON2 => println!("Win for Player 2"),
        GameState::DRAW => println!("It's a draw"),
        GameState::CONTINUE => println!("What"),
    }
}

fn step(sequence: &str, player: u8) -> GameState {
    let game_state = check_state(sequence);
    match game_state {
        GameState::CONTINUE => {
            // Check what happens if x

            let mut sequence_x = sequence.to_string();
            sequence_x.push('x');
            let opt_x = step(&sequence_x, player ^ 1);

            // Check what happens if o
            let mut sequence_o = sequence.to_string();
            sequence_o.push('o');
            let opt_o = step(&sequence_o, player ^ 1);

            // Choose the best option
            get_best(player, opt_x, opt_o)
        },
        _ => game_state
    }
}

fn check_win(sequence: &str, wincon: WinCon) -> bool {
    let sequence_length = sequence.len();
    if sequence_length < wincon.sequence_length {
        false
    } else {
        let sub_sequence = &sequence[sequence_length-wincon.sequence_length..];
        let sequence_amount = sequence.split(sub_sequence).count();
        sequence_amount > wincon.sequence_amount
    }
}

fn check_win_1(sequence: &str) -> bool {
    check_win(sequence, WinCon { sequence_length: 5, sequence_amount: 2 })
}

fn check_win_2(sequence: &str) -> bool {
    check_win(sequence, WinCon { sequence_length: 3, sequence_amount: 4 })
}

fn check_state(sequence: &str) -> GameState {
    let win_1 = check_win_1(sequence);
    let win_2 = check_win_2(sequence);
    if win_1 && win_2 {
        GameState::DRAW
    } else if win_1 {
        GameState::WON1
    } else if win_2 {
        GameState::WON2
    } else {
        GameState::CONTINUE
    }
}

fn get_best(player: u8, opt_1: GameState, opt_2: GameState) -> GameState {
    match opt_1 {
        GameState::WON1 => {
            if player == 1 {
                GameState::WON1
            } else {
                opt_2
            }
        },
        GameState::WON2 => {
            if player == 1 {
                opt_2
            } else {
                GameState::WON2
            }
        },
        _ => {
            if player == 1 {
                match opt_2 {
                    GameState::WON1 => GameState::WON1,
                    _ => GameState::DRAW,
                }
            } else {
                match opt_2 {
                    GameState::WON2 => GameState::WON2,
                    _ => GameState::DRAW,
                }
            }
        },
    }
}