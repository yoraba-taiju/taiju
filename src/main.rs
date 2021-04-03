use amethyst::utils::application_root_dir;
use amethyst::window::DisplayConfig;
use amethyst::{Application, CoreApplication, GameData, GameDataBuilder};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::audio::AudioBundle;
use amethyst::input::StringBindings;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::renderer::{RenderingBundle, RenderToWindow, RenderPbr3D, };
use amethyst::renderer::types::DefaultBackend;

mod taiju;

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;
  let assets_dir = app_root.join("assets/");

  let render_display_config = DisplayConfig {
    title: "YorabaTaiju".to_string(),
    dimensions: Some((1920, 1080)),
    ..Default::default()
  };

  let game_data = GameDataBuilder::default()
    .with_bundle(AudioBundle::default())?
    //.with_bundle(UiBundle::<StringBindings>::new())?
    .with_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
          RenderToWindow::from_config(render_display_config)
            .with_clear([0.0, 0.0, 0.0, 1.0]), //rgba background
        )
        .with_plugin(RenderPbr3D::default())
        //.with_plugin(RenderUi::default()),
    )?;


  // Set up the core application.
  let mut game: Application<GameData> =
    CoreApplication::build(assets_dir.clone(), taiju::scenes::loading::LoadingState::new())?
      .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
      .build(game_data)?;
  game.run();

  return amethyst::Result::Ok(());
}
