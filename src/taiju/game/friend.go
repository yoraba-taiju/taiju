package game

import (
	"taiju/engine"
)

type Friend engine.Actor

type FriendStrategy interface {
	Move(Friend)
}

type FriendBase struct {
	speed  engine.Vector
	damage float32
}

func NewFriendBehavior() engine.Behavior{
	behavior := &FriendBase{}
	return behavior;
}

func (FriendBase *FriendBase) Move(act engine.Actor) {
}
func (FriendBase *FriendBase) Draw(act engine.Actor, ctx *engine.DrawContext) {

}
func (FriendBase *FriendBase) OnTouch(act engine.Actor, pt *engine.Point) bool {
	return false
}
func (FriendBase *FriendBase) OnSlide(act engine.Actor, pt *engine.Point, vec *engine.Vector) bool {
	return false
}
func (FriendBase *FriendBase) OnTouchUp(act engine.Actor, pt *engine.Point) {

}
func (FriendBase *FriendBase) OnVanish(act engine.Actor) {

}
func (FriendBase *FriendBase) OnAppear(act engine.Actor) {

}
func (FriendBase *FriendBase) OnMessage(act engine.Actor, msg engine.Message) {

}
