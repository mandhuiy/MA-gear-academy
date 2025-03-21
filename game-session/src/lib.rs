#![no_std]
#![allow(warnings)]
use game_session_io::*;
use gstd::{debug, exec, msg, prelude::*};

static mut SESSION: Option<Session> = None;

const MaxTempt: u8 = 5;
