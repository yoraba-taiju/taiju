package game

import (
	"taiju/engine"
)

//弾一つで進むだけのバレット

type NormalBullet struct {
	speed  engine.Vector
	damage float32
}

func NewNormalBullet(damage float32) engine.Behavior {
	return &NormalBullet{engine.Vector{0,0}, damage}
}

func (bullet *NormalBullet) Move(act engine.Actor) {
	if !act.IsHit(act.Scene()) {
		act.Vanish()
	}
	player := act.FindActor(PlayerID)
	if player != nil {
		if player.IsHit(act) {
			player.SendMessage(NewDamageMessage(bullet.damage))
		}
	}
}
func (bullet *NormalBullet) Draw(act engine.Actor, ctx *engine.DrawContext) {

}
func (bullet *NormalBullet) OnTouch(act engine.Actor, pt *engine.Point) bool {
	return false //DO NOTHING
}
func (bullet *NormalBullet) OnSlide(act engine.Actor, pt *engine.Point, vec *engine.Vector) bool {
	return false // DO NOTHING
}
func (bullet *NormalBullet) OnTouchUp(act engine.Actor, pt*engine.Point) {
	// DO NOTHING
}
func (bullet *NormalBullet) OnAppear(act engine.Actor) {

}
func (bullet *NormalBullet) OnVanish(act engine.Actor) {

}
func (bullet *NormalBullet) OnMessage(act engine.Actor, msg engine.Message) {

}
