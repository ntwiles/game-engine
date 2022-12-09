pub mod player_movement;

use crate::{entity::Entity, state::State};

pub trait Component {
    fn update(&self, entity: &mut Entity, state: &mut State, delta_time: f64);
}
