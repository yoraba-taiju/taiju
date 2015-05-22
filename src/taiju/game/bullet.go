package game

import (
	"taiju/engine"
)

type BulletBase struct {
	engine.ActorBase
}

func NewBulletBase() BulletBase {
	return BulletBase{ ActorBase: engine.NewActorBase() }
}

//弾一つで進むだけのバレット

type NormalBullet struct {
	BulletBase
	speed  engine.Vector
	damage float32
}

func NewNormalBullet() engine.Actor {
	return &NormalBullet{ BulletBase: NewBulletBase() }
}

func (bullet *NormalBullet) Move() {
	bullet.Position_.Add(bullet.speed)
	if !bullet.IsHit(bullet.Scene()) {
		bullet.Vanish()
	}
	player := bullet.Scene().CallActor("Player")
	if player != nil{
		if player.IsHit(bullet) {
			player.SendMessage(NewDamageMessage(bullet.damage))
		}
	}
}
func (bullet *NormalBullet) Draw(*engine.DrawContext) {

}
func (bullet *NormalBullet) OnTouch(*engine.Point) bool {
	return false //DO NOTHING
}
func (bullet *NormalBullet) OnSlide(*engine.Point, *engine.Vector) bool {
	return false // DO NOTHING
}
func (bullet *NormalBullet) OnTouchUp(*engine.Point) {
	// DO NOTHING
}
func (bullet *NormalBullet) OnAppear() {

}
func (bullet *NormalBullet) OnVanish() {

}
