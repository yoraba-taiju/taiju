package game

import (
	"taiju/engine"
)

type Friend interface {
	engine.Actor
}

type FriendStrategy interface {
	Move(Friend);
}

type FriendBase struct {
	engine.ActorBase
	speed  engine.Vector
	damage float32
}

func (FriendBase *FriendBase) Move() {
}
func (FriendBase *FriendBase) Draw(*engine.DrawContext) {

}
func (FriendBase *FriendBase) OnTouch(*engine.Point) bool {
	return false
}
func (FriendBase *FriendBase) OnSlide(*engine.Point, *engine.Vector) bool {
	return false
}
func (FriendBase *FriendBase) OnTouchUp(*engine.Point) {

}
func (FriendBase *FriendBase) OnVanish() {

}
