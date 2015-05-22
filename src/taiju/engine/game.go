package engine

import (
	"github.com/go-gl/glfw/v3.1/glfw"
)

// 最上位になるクラス。
// OpenGLのマネジメントとかもする
// 継承はしない。アプリロジックはWorldに書こう。

type Game struct {
	ScreenSize Vector
	Window     *glfw.Window
}

func NewGame(world World) *Game {
	game := &Game{}
	return game
}

func (game *Game) Init(width int, height int, title string) error {
	var err error
	err = glfw.Init()
	if err != nil {
		return err
	}
	game.Window,err = glfw.CreateWindow(width, height, title, nil, nil)
	if err != nil {
		return err
	}
	return nil
}
func (game *Game) Exit() {
	game.Window.Destroy()
	glfw.Terminate()
}
func (game *Game) Run() error {
	if game.Window.ShouldClose(){
		return nil
	}
	glfw.PollEvents()
	return nil
}
