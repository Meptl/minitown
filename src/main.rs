#![deny(missing_docs)]

//! A Sudoku game.

extern crate cgmath;
extern crate ggez;
extern crate specs;
#[macro_use]
extern crate specs_derive;

mod mainstate;
mod ecs;

const NAME: &'static str = "Mini Town";

fn main() {
    let c = ggez::conf::Conf::new();
    let ctx = &mut ggez::Context::load_from_conf(NAME, "ggez", c).unwrap();
    let state = &mut mainstate::MainState::new().unwrap();
    ggez::event::run(ctx, state).unwrap();
}

