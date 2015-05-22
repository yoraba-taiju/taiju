package engine

import (

)

// 最上位になるクラス。
// OpenGLのマネジメントとかもする
// 継承はしない。アプリロジックはWorldに書こう。

type Game struct {
}

func NewGame(world World) *Game{
	game := &Game{}
	return game;
}

func (game *Game) Run(){
	
}