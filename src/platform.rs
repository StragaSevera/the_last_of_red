use bevy::prelude::*;

pub struct Platform;

#[derive(Bundle)]
pub struct PlatformBundle {
    _platform: Platform,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl PlatformBundle {
    pub fn new(pos: Transform, materials: &mut ResMut<Assets<ColorMaterial>>) -> Self {
        Self {
            _platform: Platform,
            sprite_bundle: SpriteBundle {
                material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                transform: pos,
                sprite: Sprite::new(Vec2::new(200.0, 20.0)),
                ..Default::default()
            },
        }
    }
}
