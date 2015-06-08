package game

import (
	"taiju/engine"
)

type Player struct {
	engine.MultiActorBase
	life float32
}

func NewPlayer() engine.Actor {
	player := &Player{MultiActorBase: engine.NewMultiActorBase()}
	player.SetID(PlayerID)
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
func (player *Player) OnAppear() {

}
func (player *Player) OnVanish() {

}

func (player *Player) OnMessage(msg engine.Message) {
	if damage, ok := msg.(DamageMessage); ok {
		player.life -= float32(damage)
		if player.life < 0 {
			// On Game Over
		}
	}
}
