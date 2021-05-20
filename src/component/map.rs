use bevy::prelude::*;
use crate::{model, state::MapState};

/******************************************************************************
 ** Map
 ******************************************************************************/

#[derive(Debug, Default)]
pub struct MapComponent;
#[derive(Debug, Default, Bundle)]
pub struct MapBundle {
  pub map_component: MapComponent,
  pub global_transform: GlobalTransform,
  pub transform: Transform,
}

impl MapBundle {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }
}

/******************************************************************************
 ** Course
 ******************************************************************************/

#[derive(Debug, Default)]
pub struct CourseComponent {
  pub length: usize,
}

#[derive(Debug, Default, Bundle)]
pub struct CourseBundle {
  pub course_componense: CourseComponent,
  pub global_transform: GlobalTransform,
  pub transform: Transform,
}

impl CourseBundle {
  pub fn new(length: usize) -> Self {
    Self {
      course_componense: CourseComponent {
        length,
      },
      ..Default::default()
    }
  }
}

/******************************************************************************
 ** CourseKeyframe
 ******************************************************************************/

#[derive(Debug, Default)]
pub struct CourseKeyframeComponent {
  pub at: usize,
  pub pos: Vec2,
}

#[derive(Debug, Default, Bundle)]
pub struct CourseKeyframeBundle {
  pub course_keyframe_component: CourseKeyframeComponent,
  pub global_transform: GlobalTransform,
  pub transform: Transform,
}


impl CourseKeyframeBundle {
  pub fn new(at: usize, pos: Vec2) -> Self {
    Self {
      course_keyframe_component: CourseKeyframeComponent {
        at,
        pos,
      },
      ..Default::default()
    }
  }
}

/******************************************************************************
 ** CourseCurrentFrame
 ******************************************************************************/


 #[derive(Debug, Default)]
 pub struct CourseCurrentFrameComponent {
  pub at: usize,
}

#[derive(Debug, Default, Bundle)]
pub struct CourseCurrentFrameBundle {
  pub course_current_frame_component: CourseCurrentFrameComponent,
  pub global_transform: GlobalTransform,
  pub transform: Transform,
}


impl CourseCurrentFrameBundle {
  pub fn new(at: usize) -> Self {
    Self {
      course_current_frame_component: CourseCurrentFrameComponent {
        at,
      },
      ..Default::default()
    }
  }
}

/******************************************************************************
 ** Event
 ******************************************************************************/

#[derive(Debug)]
pub struct EventComponent {
  pub at: usize,
  pub event: taiju::chapter::scenario::Event,
}

#[derive(Debug, Bundle)]
pub struct EventBundle {
  pub event_component: EventComponent,
  pub global_transform: GlobalTransform,
  pub transform: Transform,
}

impl EventBundle {
  pub fn new(at: usize, event: taiju::chapter::scenario::Event) -> Self {
    Self {
      event_component: EventComponent {
        at,
        event,
      },
      global_transform: Default::default(),
      transform: Default::default(),
    }
  }
}

/******************************************************************************
 ** Inseet/Delete
 ******************************************************************************/

pub fn insert(
  commands: &mut Commands,
  map: &model::Map,
) -> crate::state::MapState {
  let mut course_id = Entity::new(0);
  let mut current_frame_id = Entity::new(0);
  let map_id: Entity;

  map_id = commands.spawn().insert(MapComponent)
  .with_children(|builder| {
    course_id = builder.spawn()
      .insert_bundle(CourseBundle::new(map.course.length))
      .with_children(|builder|{
        for (at, pos) in &map.course.keyframes {
          builder.spawn().insert_bundle(CourseKeyframeBundle::new(*at, *pos));
        }
      }).id();
    current_frame_id = builder.spawn()
      .insert_bundle(CourseCurrentFrameBundle::new(0))
      .id();
    for event in &map.events {
      builder.spawn().insert_bundle(EventBundle::new(event.at as usize, event.event.clone()));
    }
  }).id();
  crate::state::MapState {
    map_id,
    course_id,
    current_frame_id,
  }
}

pub fn clear(
  commands: &mut Commands,
  map_state: &MapState,
) {
  commands.entity(map_state.map_id).despawn_recursive();
}