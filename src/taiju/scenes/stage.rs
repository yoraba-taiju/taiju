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

pub struct StageState {
}

impl StageState {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl SimpleState for StageState {
  fn on_start(&mut self, data: StateData<'_, GameData>) {
    let loader = data.resources.get::<DefaultLoader>().unwrap();
    let mesh_storage = data.resources.get::<ProcessingQueue<MeshData>>().unwrap();
    let tex_storage = data.resources.get::<ProcessingQueue<TextureData>>().unwrap();
    let mtl_storage = data.resources.get::<ProcessingQueue<Material>>().unwrap();
    {
      let mut transform = Transform::default();
      transform.set_translation_xyz(0.0, 0.0, -12.0);
      transform.prepend_rotation_y_axis(std::f32::consts::PI);

      let (width, height) = {
        let dim = data.resources.get::<ScreenDimensions>().unwrap();
        (dim.width(), dim.height())
      };

      data.world.extend(vec![(
        Camera::standard_3d(width, height),
        transform,
      )]);
    }

    {
      let mut transform = Transform::default();
      transform.set_translation_xyz(6.0, 6.0, -6.0);

      let light: Light = PointLight {
        intensity: 5.0,
        color: Srgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
      }.into();

      data.world.extend(vec![(
        light,
        transform,
      )]);
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
      pos.set_translation_xyz(0.0, 0.0, 10.0);
      data.world.extend(vec![(
        pos,
        mesh.clone(),
        mtl,
      )]);
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
      pos.set_translation_xyz(0.0, 0.0, -1.0);
      data.world.extend(vec![(
        pos,
        mesh,
        mtl,
      )]);
    }
  }

  fn fixed_update(&mut self, _data: StateData<'_, GameData>) -> SimpleTrans {
    Trans::None
  }
}
