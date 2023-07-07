use taiju::build_app;

fn main() -> anyhow::Result<()> {
  build_app().run();
  Ok(())
}
