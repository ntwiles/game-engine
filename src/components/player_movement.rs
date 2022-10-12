use cgmath::prelude::*;

use crate::{
    entity::Entity,
    graphics::{sprite, vertex},
    state::State,
};

use super::Component;

pub struct PlayerMovement {}

impl Component for PlayerMovement {
    fn update(&self, entity: &mut Entity, state: &mut State) {
        let mut movement = cgmath::Vector2::<f32>::zero();

        if state.is_left_pressed {
            movement.x = -0.07;
        } else if state.is_right_pressed {
            movement.x = 0.07;
        }

        if state.is_up_pressed {
            movement.y = 0.07;
        } else if state.is_down_pressed {
            movement.y = -0.07;
        }

        if movement.is_zero() {
            return;
        }

        if let Some(player_coll) = &entity.collider {
            if let Some(wall) = &state.wall {
                if let Some(other_coll) = &wall.collider {
                    if player_coll.cast(
                        entity.get_position() + movement,
                        &other_coll,
                        wall.get_position(),
                    ) {
                        movement = cgmath::Vector2 { x: 0.0, y: 0.0 }
                    }
                }
            }
        }

        entity.move_by(movement);

        let verts = vertex::RenderVertex::new(
            entity.get_position(),
            entity.get_rotation(),
            &sprite::Sprite::get_vertices(),
        );

        state.graphics.write_entity(entity.get_id(), verts);
        state.camera.set_position(entity.get_position());
    }
}
