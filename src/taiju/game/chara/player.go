package chara

import (
	"taiju/engine"
	"taiju/game/id"
	"taiju/game/msg"
)

type Player struct {
	engine.MultiActorBase
	life float32
}

func NewPlayer() engine.Behavior {
	player := &Player{}
	player.SetID(id.Player)
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

func (player *Player) OnMessage(act engine.Actor,message engine.Message) {
	if damage, ok := message.(msg.DamageMessage); ok {
		player.life -= float32(damage)
		if player.life < 0 {
			// On Game Over
		}
	}
}
