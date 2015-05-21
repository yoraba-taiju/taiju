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
	OnVanish()
	OnMessage(Message)
}

// ベースとなる、一応基本的な処理は備えているクラス
type ActorBase struct {
	name         string
	drawLevel    float32
	scene        *Scene
	SelfPosition Point
	SelfSize     Vector
	alive        bool
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
	return self.SelfPosition
}
func (self *ActorBase) Size() Vector {
	return self.SelfSize
}
func (self *ActorBase) IsHit(other Actor) bool {
	return IsHitBox(self.SelfPosition, self.SelfSize, other.Position(), other.Size())
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
