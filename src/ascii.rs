use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub fn load_ascii_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Ascii.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(9.0), 16, 16, Vec2::splat(2.0));
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    assert!(index < 256, "Index out of Ascii Range");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}
