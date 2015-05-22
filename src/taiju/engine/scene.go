package engine

import (
	"errors"
	"math"
	"sort"
)

var SceneNameIsConstantError = errors.New("Name of Scene is Constant")

type Scene struct {
	size     Vector
	actors   []Actor
	actorMap map[string]Actor
	vanished bool
	affine   Affine
}

func (scene *Scene) AddActor(actor Actor) {
	scene.actors = append(scene.actors)
	name := actor.Name()
	if len(name) > 0 {
		scene.actorMap[name] = actor
	}
}

func (self *Scene) Name() string {
	return "Scene"
}
func (self *Scene) SetName(name string) error {
	return SceneNameIsConstantError
}
func (self *Scene) DrawLevel() float32 {
	return float32(math.NaN())
}
func (self *Scene) SetDrawLevel(level float32) {
}
func (scene *Scene) Scene() *Scene {
	return scene
}
func (scene *Scene) Position() Point {
	return Point{0, 0}
}
func (scene *Scene) Affine() Affine {
	return scene.affine
}
func (scene *Scene) Size() Vector {
	return scene.size
}
func (scene *Scene) IsHit(other Actor) bool {
	return IsHitBox(Point{0, 0}, scene.size, other.Position(), other.Size())
}
func (scene *Scene) Vanish() {
	//TODO
}
func (scene *Scene) IsAlive() bool {
	//TODO
	return true
}
func (scene *Scene) OnTouch(*Point) bool {
	//TODO
	return true
}
func (scene *Scene) OnSlide(*Point, *Vector) bool {
	//TODO
	return true
}
func (scene *Scene) OnTouchUp(*Point) {
	//TODO
}
func (scene *Scene) OnVanish() {
	//TODO
}
func (scene *Scene) OnAppear() {
	//TODO
}

func (scene *Scene) CallActor(name string) Actor {
	actor, ok := scene.actorMap[name]
	if ok {
		return actor
	} else {
		return nil
	}
}
func (scene *Scene) SendMessage(msg Message) {
	scene.OnMessage(msg)
}
func (scene *Scene) OnMessage(msg Message) {

}
func (scene *Scene) Move() {
	for _, actor := range scene.actors {
		actor.Move()
	}
	for scene.vanished {
		scene.vanished = false
		next := make([]Actor, 0, len(scene.actors))
		for _, actor := range scene.actors {
			if actor.IsAlive() {
				next = append(next, actor)
			} else {
				actor.OnVanish()
				name := actor.Name()
				if len(name) > 0 {
					delete(scene.actorMap, name)
				}
			}
		}
		scene.actors = next
	}
}

type DrawSort []Actor

func (self DrawSort) Len() int {
	return len(self)
}
func (self DrawSort) Less(i, j int) bool {
	return self[i].DrawLevel() < self[j].DrawLevel()
}
func (self DrawSort) Swap(i, j int) {
	self[i], self[j] = self[j], self[i]
}
func (scene *Scene) Draw(ctx *DrawContext) {
	sort.Sort(DrawSort(scene.actors))
	for _, actor := range scene.actors {
		actor.Draw(ctx)
	}
}
