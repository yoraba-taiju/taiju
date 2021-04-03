use amethyst::utils::application_root_dir;
use amethyst::window::DisplayConfig;
use amethyst::{Application, CoreApplication, GameData};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::renderer::{RenderingBundle, RenderToWindow, RenderPbr3D, RenderShaded3D};
use amethyst::renderer::types::DefaultBackend;
use amethyst::ecs::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::rendy::core::hal::command::ClearColor;
use amethyst::assets::LoaderBundle;

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

  let mut builder = DispatcherBuilder::default();
  builder
//    .with_bundle(AudioBundle::default())?
    //.with_bundle(UiBundle::<StringBindings>::new())?
    .add_bundle(LoaderBundle)
    .add_bundle(TransformBundle)
    .add_bundle(
      RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
          RenderToWindow::from_config(render_display_config)
            .with_clear(ClearColor{
              float32: [0.0, 0.0, 0.0, 1.0]
            }), //rgba background
        )
        .with_plugin(RenderShaded3D::default())
        //.with_plugin(RenderUi::default()),
    );


  // Set up the core application.
  let mut game: Application<GameData> =
    CoreApplication::build(assets_dir.clone(), taiju::scenes::loading::LoadingState::new())?
      .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
      .build(builder)?;
  game.run();

  return amethyst::Result::Ok(());
}
