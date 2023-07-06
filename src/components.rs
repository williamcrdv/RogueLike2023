use specs_derive::Component;
use specs::prelude::*;
use rltk::{Rltk, GameState, RGB};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Player {}


#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range: i32,
    pub dirty : bool
}

#[derive(Component)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name : String
}

