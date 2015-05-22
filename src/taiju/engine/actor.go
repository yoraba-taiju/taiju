package engine

import ()

type Message interface {
}

type Actor interface {
	Scene() *Scene
	SendMessage(Message)
	//
	Name() string
	SetName(string) error
	//
	DrawLevel() float32
	SetDrawLevel(float32)
	//
	Position() Point
	Size() Vector
	Affine() Affine
	IsHit(Actor) bool
	//
	Move()
	Draw(*DrawContext)
	//
	Vanish()
	IsAlive() bool
	//
	OnTouch(*Point) bool
	OnSlide(*Point, *Vector) bool
	OnTouchUp(*Point)
	//
	OnAppear()
	OnVanish()
	OnMessage(Message)
}

// ベースとなる、一応基本的な処理は備えているクラス
type ActorBase struct {
	name      string
	drawLevel float32
	scene     *Scene
	Affine_   Affine
	Position_ Point
	Size_     Vector
	alive     bool
}

func NewActorBase() ActorBase {
	return ActorBase{}
}
func (self *ActorBase) Name() string {
	return self.name
}
func (self *ActorBase) SetName(name string) error {
	self.name = name
	return nil
}
func (self *ActorBase) DrawLevel() float32 {
	return self.drawLevel
}
func (self *ActorBase) SetDrawLevel(drawLevel float32) {
	self.drawLevel = drawLevel
}

func (self *ActorBase) Scene() *Scene {
	return self.scene
}
func (self *ActorBase) Position() Point {
	return self.Position_
}
func (self *ActorBase) Size() Vector {
	return self.Size_
}
func (self *ActorBase) Affine() Affine {
	return self.Affine_
}
func (self *ActorBase) IsHit(other Actor) bool {
	return IsHitBox(self.Position_, self.Size_, other.Position(), other.Size())
}
func (self *ActorBase) SendMessage(msg Message) {
	self.OnMessage(msg)
}
func (self *ActorBase) OnMessage(msg Message) {

}

func (self *ActorBase) Vanish() {
	self.scene.vanished = true
	self.alive = false
}
func (self *ActorBase) IsAlive() bool {
	return self.alive
}

//
type MultiActor interface {
	Actor
	AddChild(int, Actor)
	GetChild(int) Actor
	Children() []Actor
}
type MultiActorBase struct {
	ActorBase
	children map[int]Actor
}

func NewMultiActorBase() MultiActorBase {
	return MultiActorBase{}
}
func (self *MultiActorBase) Move() {
	for _, v := range self.children {
		v.Move()
	}
}
func (self *MultiActorBase) Draw(context *DrawContext) {
	for _, v := range self.children {
		v.Draw(context)
	}
}
