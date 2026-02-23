//todo health! this will end up being like health bars that appear above the
//entities and what not.
use bevy::prelude::*;

#[derive(Component)]
pub struct Health{
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(max: u32) -> Self {
        Self { current: max, max } //fancy syntax hehe 
    }
}