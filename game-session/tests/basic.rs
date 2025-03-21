#![no_std]

use game_session_io::*;
use gtest::{Program, ProgramBuilder, System};

const USER1: u64 = 10;
const SESSION_PROGRAM_ID: u64 = 100;
const WORDLE_PROGRAM_ID: u64 = 200;

#[test]
fn test_game_start() {
    let system = System::new();
    system.init_logger();

    let session_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);

    let wordle_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    system.mint_to(USER1, 1000000000000000);
    wordle_program.send_bytes(USER1, []);
    system.run_next_block();

    session_program.send(USER1, wordle_program.id());
    system.run_next_block();

    session_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    system.run_next_block();

    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "helle".into(),
        },
    );
    system.run_next_block();

    let state: Session = session_program
        .read_state(())
        .expect("Failed to read state");
    assert_eq!(state.session_status, SessionStatus::Waiting);
}

