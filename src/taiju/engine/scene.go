package engine

import (
)

type Scene interface {
	MultiActor
}

type SceneBehavior interface {
	Behavior
}

func NewScene(w World, behavior SceneBehavior) Scene{
	scene := SceneBase{
		MultiActorBase: newMultiActorBase(nil, nil, behavior),
	}
	scene.parent = &scene
	scene.scene = &scene
	return &scene;
}

type SceneBase struct {
	MultiActorBase
	world             World
	isSomeOneVanished bool
}

func (scene *SceneBase) NotifyChildDeath() {
	scene.isSomeOneVanished = true;
}

func (scene *SceneBase) Scene() Scene {
	return scene
}

func (scene *SceneBase) Vanish() {
	scene.world.NotifySceneDeath(scene)
	scene.setAlive(false)
}

func (scene *SceneBase) Move() {
	scene.MultiActorBase.Move()
	for scene.isSomeOneVanished {
		scene.isSomeOneVanished = false
		scene.InterDeadChildren()
	}
}
