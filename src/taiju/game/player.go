package game

import (
	"taiju/engine"
)

type Player struct {
	engine.ActorBase
	life float32
}

func NewPlayer() engine.Actor {
	player := &Player{}
	player.SetName("Player")
	player.life = 100
	return player
}

func (player *Player) Move() {
}

func (player *Player) Draw(*engine.DrawContext) {

}
func (player *Player) OnTouch(*engine.Point) bool {
	return false
}
func (player *Player) OnSlide(*engine.Point, *engine.Vector) bool {
	return false
}
func (player *Player) OnTouchUp(*engine.Point) {

}
func (player *Player) OnVanish() {

}
