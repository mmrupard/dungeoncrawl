pub use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Enemy;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToAttackk {
    pub attacker: Entity,
    pub opponent: Entity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChasingPlayer;
