package game

import (
	"taiju/engine"
)

type World struct {
	engine.WorldBase
}

func NewWorld() engine.World {
	world := &World{
		WorldBase: engine.NewWorldBase(60),
	}
	return world
}
