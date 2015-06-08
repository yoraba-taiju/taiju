package game

import (
	"taiju/engine"
)

type Player struct {
	engine.MultiActorBase
	life float32
}

func NewPlayer() engine.Behavior {
	player := &Player{}
	player.SetID(PlayerID)
	player.life = 100
	return player
}

func (player *Player) Move(act engine.Actor) {
}

func (player *Player) Draw(act engine.Actor,ctx *engine.DrawContext) {

}
func (player *Player) OnTouch(act engine.Actor,pt *engine.Point) bool {
	return false
}
func (player *Player) OnSlide(act engine.Actor, pt*engine.Point, vec *engine.Vector) bool {
	return false
}
func (player *Player) OnTouchUp(act engine.Actor, pt *engine.Point) {

}
func (player *Player) OnAppear(act engine.Actor) {

}
func (player *Player) OnVanish(act engine.Actor) {

}

func (player *Player) OnMessage(act engine.Actor,msg engine.Message) {
	if damage, ok := msg.(DamageMessage); ok {
		player.life -= float32(damage)
		if player.life < 0 {
			// On Game Over
		}
	}
}
