use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;
use bevy::prelude::*;
use bitmask_enum::bitmask;
use static_init::dynamic;
use std::collections::{HashMap, HashSet};

use crate::platform::Platform;

pub struct Player {
    velocity: Vec3,
    state: PlayerState,
}

#[bitmask(u8)]
pub enum PlayerState {
    MovingLeft,
    MovingRight,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    _player: Player,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(materials: &mut ResMut<Assets<ColorMaterial>>) -> Self {
        Self {
            _player: Player {
                velocity: Vec3::ZERO,
                state: PlayerState::none(),
            },
            sprite_bundle: SpriteBundle {
                material: materials.add(Color::rgb(0.9, 0.0, 0.0).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                sprite: Sprite::new(Vec2::new(40.0, 40.0)),
                ..Default::default()
            },
        }
    }
}

const RUN_SPEED: f32 = 200.0;
const FALL_SPEED: f32 = 300.0;

pub fn player_input_system(mut keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Player>) {
    if let Ok(mut player) = query.single_mut() {}
}

// pub fn player_fall_system(
//     time: Res<Time>,
//     mut player_query: Query<(&mut Player, &mut Transform), Without<Platform>>,
//     collider_query: Query<(&Platform, &Transform, &Sprite)>,
// ) {
//     // if let Ok((_, mut transform)) = player_query.single_mut() {
//     //     let translation = &mut transform.translation;
//     //     translation.y -= time.delta_seconds() * FALL_SPEED;
//     // }
// }

//         let mut delta = 0.0;
//
//         let input = keyboard_input
//             .iter()
//             .filter(|ev| ev.key_code.is_some() && RUN_CODES.contains_key(&ev.key_code.unwrap()));
//
//         for event in input {
//             let mut nudge = RUN_CODES[&event.key_code.unwrap()].to_owned();
//             if event.state == ElementState::Released {
//                 nudge *= -1.0;
//             }
//
//             delta += nudge;
//         }
//
//         player.velocity.x += delta;

pub fn player_move_system(time: Res<Time>, mut query: Query<(&Player, &mut Transform)>) {
    if let Ok((player, mut transform)) = query.single_mut() {
        let delta = player.velocity * time.delta_seconds();
        transform.translation += delta;
    }
}
