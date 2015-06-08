package engine

import (
	"sort"
)

type Message interface {
}

type Actor interface {
	Scene() Scene
	Parent() MultiActor
	SendMessage(Message)
	//
	ID() int
	SetID(int)
	FindActor(int) Actor
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
	setAlive(bool)
	//
	OnTouch(*Point) bool
	OnSlide(*Point, *Vector) bool
	OnTouchUp(*Point)
	//
	OnAppear()
	OnVanish()
	OnMessage(Message)
}
type Behavior interface {
	Move(Actor)
	Draw(Actor, *DrawContext)
	//
	OnTouch(Actor, *Point) bool
	OnSlide(Actor, *Point, *Vector) bool
	OnTouchUp(Actor, *Point)
	//
	OnAppear(Actor)
	OnVanish(Actor)
	OnMessage(Actor, Message)
}

// ベースとなる、一応基本的な処理は備えているクラス
type ActorBase struct {
	id        int
	drawLevel float32
	scene     Scene
	parent    MultiActor
	Affine_   Affine
	Position_ Point
	Size_     Vector
	alive     bool
	behavior  Behavior
}

func NewActor(scene Scene, parent MultiActor, behavior Behavior) Actor{
	actor := newActorBase(scene, parent, behavior)
	return &actor;
}
func newActorBase(scene Scene, parent MultiActor, behavior Behavior) ActorBase{
	return ActorBase{
		id: -1,
		drawLevel: 0,
		scene: scene,
		parent: parent,
		Affine_: nil,
		Size_: Vector{0,0},
		alive: true,
		behavior: behavior,
	}
}
func newMultiActorBase(scene Scene, parent MultiActor, behavior Behavior) MultiActorBase{
	return MultiActorBase{
		ActorBase: newActorBase(scene, parent, behavior),
	}
}
func NewMultiActor(scene Scene, parent MultiActor, behavior Behavior) MultiActor{
	p := newMultiActorBase(scene, parent, behavior)
	return &p
}

func NewActorBase() ActorBase {
	return ActorBase{}
}

func (self *ActorBase) ID() int {
	return self.id
}
func (self *ActorBase) SetID(id int) {
	self.id = id
}
func (self *ActorBase) FindActor(id int) Actor {
	if self.id == id {
		return self
	} else {
		return nil
	}
}

func (self *ActorBase) DrawLevel() float32 {
	return self.drawLevel
}
func (self *ActorBase) SetDrawLevel(drawLevel float32) {
	self.drawLevel = drawLevel
}

func (self *ActorBase) Scene() Scene {
	return self.scene
}
func (self *ActorBase) Parent() MultiActor {
	return self.parent
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

func (self *ActorBase) Vanish() {
	self.setAlive(false)
	self.parent.NotifyChildDeath()
}
func (self *ActorBase) IsAlive() bool {
	return self.alive
}
func (self *ActorBase) setAlive(b bool) {
	self.alive = b
}

//
func (self *ActorBase) Move() {
	self.behavior.Move(self)
}
func (self *ActorBase) Draw(ctx *DrawContext) {
	self.behavior.Draw(self, ctx)
}
func (self *ActorBase) OnTouch(pt *Point) bool {
	return self.behavior.OnTouch(self, pt)
}
func (self *ActorBase) OnSlide(pt *Point, vec *Vector) bool {
	return self.behavior.OnSlide(self, pt, vec)
}
func (self *ActorBase) OnTouchUp(pt *Point) {
	self.behavior.OnTouchUp(self, pt)
}
func (self *ActorBase) OnAppear() {
	self.behavior.OnAppear(self)
}
func (self *ActorBase) OnVanish() {
	self.behavior.OnVanish(self)
}
func (self *ActorBase) OnMessage(msg Message) {
	self.behavior.OnMessage(self, msg)
}

//
type MultiActor interface {
	Actor
	//
	AddActor(Actor) int
	GetActor(int) Actor
	Children() []Actor
	//
	InterDeadChildren()
	NotifyChildDeath()
}
type MultiActorBase struct {
	ActorBase
	children []Actor
}

func NewMultiActorBase() MultiActorBase {
	return MultiActorBase{}
}
func (actor *MultiActorBase) NotifyChildDeath() {
	actor.parent.NotifyChildDeath()
}

func (mact *MultiActorBase) AddActor(actor Actor) int {
	mact.children = append(mact.children, actor)
	return len(mact.children) - 1
}

func (mact *MultiActorBase) GetActor(idx int) Actor {
	return mact.children[idx]
}
func (mact *MultiActorBase) Children() []Actor {
	return mact.children
}

func (actor *MultiActorBase) Move() {
	actor.behavior.Move(actor)
	for _, v := range actor.children {
		v.Move()
	}
}
func (actor *MultiActorBase) InterDeadChildren() {
	if actor.IsAlive() {
		next := make([]Actor, 0, len(actor.children))
		for _, actor := range actor.children {
			if actor.IsAlive() {
				next = append(next, actor)
			} else {
				actor.OnVanish()
			}
		}
		actor.children = next
	} else {
		for _, child := range actor.children {
			child.OnVanish()
		}
		actor.OnVanish()
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

func (actor *MultiActorBase) Draw(context *DrawContext) {
	actor.behavior.Draw(actor, context)
	sort.Sort(DrawSort(actor.children))
	for _, actor := range actor.children {
		actor.Draw(context)
	}
}
