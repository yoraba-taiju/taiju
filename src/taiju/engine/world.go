package engine

import ()

//Sceneを動かすための舞台装置
//

type World interface {
	Fps() float32
	Move(float32) (bool, error)
	Render(DrawContext) error
	ChangeScene(Scene)
	NotifySceneDeath(Scene)
	NewScene(SceneBehavior) Scene
}

type WorldBase struct {
	DesiredFPS float32
	currentScene Scene
}

func NewWorldBase(fps float32) WorldBase {
	return WorldBase{
		DesiredFPS: fps,
	}
}

func (base *WorldBase) Move(diffTime float32) (bool, error) {
	return true, nil
}
func (base *WorldBase) Fps() float32 {
	return base.DesiredFPS
}
func (base *WorldBase) Render(ctx DrawContext) error {
	return nil
}

/**
 *  Sceme Life Cycle
 */

func (base *WorldBase) NewScene(sb SceneBehavior) Scene{
	return NewScene(base, sb)
}
func (base *WorldBase) ChangeScene(scene Scene) {
	if base.currentScene != nil{
		base.currentScene.Vanish()
	}
	base.currentScene = scene
	scene.OnAppear()
}
func (base *WorldBase) NotifySceneDeath(scene Scene) {
	base.currentScene = nil
}
