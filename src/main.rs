use bevy::prelude::*;
use bevy::core::CorePlugin;
use bevy::input::InputPlugin;
use bevy::window::WindowPlugin;
use bevy::log::LogPlugin;
use bevy::asset::AssetPlugin;
use bevy::render::RenderPlugin;
use bevy::sprite::SpritePlugin;
use bevy::winit::WinitPlugin;
use bevy::wgpu::WgpuPlugin;
use bevy::text::TextPlugin;
use bevy::scene::ScenePlugin;

mod taiju;

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .add_startup_system(taiju::scenes::stage::setup.system())
    .add_system(taiju::scenes::stage::systems::enumrate.system())
    .run();
}
