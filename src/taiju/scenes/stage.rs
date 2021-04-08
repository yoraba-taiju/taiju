use amethyst::{
  assets::{DefaultLoader, Handle, Loader, LoaderBundle, ProcessingQueue},
  core::{
    ecs::*,
    transform::{Transform, TransformBundle},
  },
  renderer::{
    camera::Camera,
    light::{Light, PointLight},
    mtl::{Material, MaterialDefaults},
    palette::{LinSrgba, Srgb},
    plugins::{RenderPbr3D, RenderToWindow},
    rendy::{
      hal::command::ClearColor,
      mesh::{Normal, Position, Tangent, TexCoord},
      texture::palette::load_from_linear_rgba,
    },
    shape::Shape,
    types::{DefaultBackend, MeshData, TextureData},
    Mesh, RenderingBundle, Texture,
  },
  utils::application_root_dir,
  window::ScreenDimensions,
  Application, GameData, SimpleState, SimpleTrans, StateData, Trans,
};
use amethyst::ecs::query::Query;

pub mod components;
pub mod systems;

pub struct StageState {
  schedule: Schedule,
}

impl StageState {
  pub fn new() -> Self {
    Self {
      schedule: Schedule::builder().build(),
    }
  }
}

impl SimpleState for StageState {
  fn on_start(&mut self, data: StateData<'_, GameData>) {
    {
      let mut schedule = Schedule::builder()
        .add_system(SystemBuilder::new("TestSystem")
          .read_component::<Transform>()
          .build(
            move |commands, world, resource, queries| {
              let mut q = <(Entity, Read<Transform>)>::query().filter(!component::<()>());
              for (entity, pos) in q.iter_mut(world) {
                //println!("{:?}: {:?}", entity, pos)
              }
            },
          ))
        .build();
      std::mem::swap(&mut self.schedule, &mut schedule);
    }
    let loader = data.resources.get::<DefaultLoader>().unwrap();
    let mesh_storage = data.resources.get::<ProcessingQueue<MeshData>>().unwrap();
    let tex_storage = data.resources.get::<ProcessingQueue<TextureData>>().unwrap();
    let mtl_storage = data.resources.get::<ProcessingQueue<Material>>().unwrap();
    {
      let mut transform = Transform::default();
      transform.set_translation_xyz(0.0, 0.0, 12.0);

      let (width, height) = {
        let dim = data.resources.get::<ScreenDimensions>().unwrap();
        (dim.width(), dim.height())
      };

      data.world.push((
        Camera::standard_3d(width, height),
        transform,
      ));
    }

    {
      let mut transform = Transform::default();
      transform.set_translation_xyz(6.0, 6.0, 6.0);

      let light: Light = PointLight {
        intensity: 5.0,
        color: Srgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
      }.into();

      data.world.push((
        light,
        transform,
      ));
    }

    {
      let mat_defaults = data.resources.get::<MaterialDefaults>().unwrap().0.clone();
      let mesh: Handle<Mesh> = loader.load_from_data(
        Shape::Cube
          .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((100.0, 100.0, 0.01)))
          .into(),
        (),
        &mesh_storage,
      );
      let albedo = loader.load_from_data(
        load_from_linear_rgba(LinSrgba::new(0.5, 0.5, 0.5, 0.5)).into(),
        (),
        &tex_storage,
      );
      let mtl: Handle<Material> = {
        let metallic_roughness = loader.load_from_data(
          load_from_linear_rgba(LinSrgba::new(0.0, 0.0, 0.0, 0.0)).into(),
          (),
          &tex_storage,
        );

        loader.load_from_data(
          Material {
            albedo: albedo,
            metallic_roughness,
            ..mat_defaults.clone()
          },
          (),
          &mtl_storage,
        )
      };
      let mut pos = Transform::default();
      pos.set_translation_xyz(0.0, 0.0, -14.0);
      data.world.push((
        pos,
        mesh.clone(),
        mtl,
      ));
    }

    {
      let mat_defaults = data.resources.get::<MaterialDefaults>().unwrap().0.clone();
      let mesh: Handle<Mesh> = loader.load_from_data(
        Shape::Cube
          .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((1.0, 1.0, 0.01)))
          .into(),
        (),
        &mesh_storage,
      );
      let albedo = loader.load_from_data(
        load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
        (),
        &tex_storage,
      );
      let mtl: Handle<Material> = {
        let metallic_roughness = loader.load_from_data(
          load_from_linear_rgba(LinSrgba::new(0.0, 0.5, 0.5, 0.0)).into(),
          (),
          &tex_storage,
        );

        loader.load_from_data(
          Material {
            albedo: albedo,
            metallic_roughness,
            ..mat_defaults.clone()
          },
          (),
          &mtl_storage,
        )
      };
      let mut pos = Transform::default();
      pos.set_translation_xyz(0.0, 0.0, 1.0);
      data.world.push((
        pos,
        mesh,
        mtl,
      ));
    }
  }

  fn fixed_update(&mut self, _data: StateData<'_, GameData>) -> SimpleTrans {
    Trans::None
  }

  fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
    self.schedule.execute(data.world, data.resources);
    Trans::None
  }
}
