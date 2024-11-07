use gtest::{Program, System};
use pebbles_game_io::*:;

#[test]
fn test(){
    let sys = System::new();
    
    // Load the program
    let program = Program::current(&sys);

    // Initialize the game with 15 pebbles and a max of 3 per turn on Easy difficulty
    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    let result = program.send("init", init_data);
    assert!(result.is_ok(), "Initialization failed");
}

#[test]
fn test_user_turn() {
    let sys = System::new();
    let program = Program::current(&sys);

    // Initialize the game
    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };
    program.send("init", init_data).expect("Initialization failed");

    // User takes a turn by removing 2 pebbles
    let action = PebblesAction::Turn(2);
    let result = program.send("handle", action);
    assert!(result.is_ok(), "User's turn failed");

    // Check the game state to ensure pebbles were removed
    let state: GameState = program.query_state("state").expect("Failed to get state");
    assert_eq!(state.pebbles_remaining, 8, "Pebbles count did not update correctly");
}

#[test]
fn test_program_turn_and_winning() {
    let sys = System::new();
    let program = Program::current(&sys);

    // Initialize with just 1 pebble to check if Program wins immediately when it goes first
    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 1,
        max_pebbles_per_turn: 3,
    };
    program.send("init", init_data).expect("Initialization failed");

    // Check if the game is already won by the Program
    let state: GameState = program.query_state("state").expect("Failed to get state");
    assert_eq!(state.winner, Some(Player::Program), "Program should win immediately");
}

#[test]
fn test_give_up() {
    let sys = System::new();
    let program = Program::current(&sys);

    // Initialize the game
    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };
    program.send("init", init_data).expect("Initialization failed");

    // User decides to give up
    let action = PebblesAction::GiveUp;
    let result = program.send("handle", action);
    assert!(result.is_ok(), "Give up action failed");

    // Check that Program is the winner
    let state: GameState = program.query_state("state").expect("Failed to get state");
    assert_eq!(state.winner, Some(Player::Program), "Program should be the winner after user gives up");
}

#[test]
fn test_restart() {
    let sys = System::new();
    let program = Program::current(&sys);

    // Initialize and play a bit
    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };
    program.send("init", init_data).expect("Initialization failed");

    // User removes 3 pebbles
    let action = PebblesAction::Turn(3);
    program.send("handle", action).expect("User's turn failed");

    // Restart the game with new settings
    let restart_action = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 20,
        max_pebbles_per_turn: 4,
    };
    program.send("handle", restart_action).expect("Restart failed");

    // Check that game state has been reset
    let state: GameState = program.query_state("state").expect("Failed to get state");
    assert_eq!(state.pebbles_remaining, 20, "Pebbles count did not reset correctly");
    assert_eq!(state.max_pebbles_per_turn, 4, "Max pebbles per turn did not reset correctly");
    assert_eq!(state.difficulty, DifficultyLevel::Hard, "Difficulty level did not reset correctly");
    assert!(state.winner.is_none(), "Winner should be none after restart");
}
Explanation of Each Test
test_initialization:
Initializes the game and ensures there’s no error.
test_user_turn:
Checks if the User can take a valid turn and that the pebbles_remaining count updates correctly.
test_program_turn_and_winning:
Initializes the game with only one pebble to check if the Program wins immediately when it goes first.
test_give_up:
Verifies that if the User gives up, the Program is declared the winner.
test_restart:
Ensures that restarting the game properly resets the state with new initialization values.
Notes
Assertions: Each test contains assertions to validate expected outcomes.
Querying State: The program.query_state("state") line is used to fetch the current game state to verify it matches expectations after actions are performed.
These tests cover various scenarios for the Pebbles game, including both expected gameplay and edge cases like immediate winning or restarting.







Dijiste:
quita los comentarios
ChatGPT dijo:
ChatGPT
rust
Copiar código
use gtest::{Program, System};
use pebbles_game_io::{PebblesInit, PebblesAction, PebblesEvent, Player, DifficultyLevel, GameState};

#[test]
fn test_initialization() {
    let sys = System::new();
    let program = Program::current(&sys);

    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };

    let result = program.send("init", init_data);
    assert!(result.is_ok());
}

#[test]
fn test_user_turn() {
    let sys = System::new();
    let program = Program::current(&sys);

    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };
    program.send("init", init_data).expect("Initialization failed");

    let action = PebblesAction::Turn(2);
    let result = program.send("handle", action);
    assert!(result.is_ok());

    let state: GameState = program.query_state("state").expect("Failed to get state");
    assert_eq!(state.pebbles_remaining, 8);
}

#[test]
fn test_program_turn_and_winning() {
    let sys = System::new();
    let program = Program::current(&sys);

    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 1,
        max_pebbles_per_turn: 3,
    };
    program.send("init", init_data).expect("Initialization failed");

    let state: GameState = program.query_state("state").expect("Failed to get state");
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_give_up() {
    let sys = System::new();
    let program = Program::current(&sys);

    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };
    program.send("init", init_data).expect("Initialization failed");

    let action = PebblesAction::GiveUp;
    let result = program.send("handle", action);
    assert!(result.is_ok());

    let state: GameState = program.query_state("state").expect("Failed to get state");
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_restart() {
    let sys = System::new();
    let program = Program::current(&sys);

    let init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };
    program.send("init", init_data).expect("Initialization failed");

    let action = PebblesAction::Turn(3);
    program.send("handle", action).expect("User's turn failed");

    let restart_action = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 20,
        max_pebbles_per_turn: 4,
    };
    program.send("handle", restart_action).expect("Restart failed");

    let state: GameState = program.query_state("state").expect("Failed to get state");
    assert_eq!(state.pebbles_remaining, 20);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert!(state.winner.is_none());
}