use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::BevyDefault,
    },
    window::PrimaryWindow,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<Red>()
        .add_startup_systems((spawn_camera, spawn_sprite))
        .add_systems((update_red, update_sprites).chain())
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(window) = window_query.get_single() {
        commands.spawn(Camera2dBundle {
            transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.5, 0.0),
            ..default()
        });
    }
}

fn create_texture(
    width: u32,
    height: u32,
    rgba_data: Vec<u8>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<Image> {
    let image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba_data,
        TextureFormat::bevy_default(),
    );
    images.add(image)
}

#[derive(Resource, Default)]
pub struct Red(u8);

impl Red {
    pub fn next(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
    pub fn get(&self) -> u8 {
        self.0
    }
}

pub fn update_red(mut red: ResMut<Red>) {
    red.next();
}

pub fn update_sprites(
    mut sprite_query: Query<&mut Handle<Image>>,
    red: Res<Red>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut texture = vec![0u8; 10 * 10 * 4];
    for r in texture.iter_mut().step_by(4) {
        *r = red.get();
    }
    for a in texture.iter_mut().skip(3).step_by(4) {
        *a = 255;
    }

    for mut sprite in sprite_query.iter_mut() {
        *sprite = create_texture(10, 10, texture.clone(), &mut images)
    }
}

pub fn spawn_sprite(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Ok(window) = window_query.get_single() {
        let mut rgba_data = vec![0u8; 10 * 10 * 4];
        for a in rgba_data.iter_mut().skip(3).step_by(4) {
            *a = 255;
        }
        let texture_handle = create_texture(10, 10, rgba_data, &mut images);

        let sprite = SpriteBundle {
            transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.5, 0.0),
            texture: texture_handle,
            ..default()
        };
        commands.spawn(sprite);
    }
}

#[test]
fn image_format() {
    assert_eq!(TextureFormat::bevy_default(), TextureFormat::Rgba8UnormSrgb)
}
