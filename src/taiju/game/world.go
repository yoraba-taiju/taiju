package game

import (
	"taiju/engine"
)

type World struct{
	engine.WorldBase
}

func NewWorld() engine.World{
	world := &World{}
	return world
}