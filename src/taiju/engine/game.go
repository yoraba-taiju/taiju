package engine

import (
	"github.com/go-gl/glfw/v3.1/glfw"
	"log"
	"time"
)

/**
 * 最上位になるクラス。
 *  - OpenGLのマネジメント
 *  - 時間管理
 * などなど、外部とゲーム内部の橋渡しを行うイメージ。
 */

type Game struct {
	ScreenSize  Vector
	Window      *glfw.Window
	World       World
	drawContext DrawContext
}

func NewGame(world World) *Game {
	game := &Game{}
	game.World = world
	return game
}

func (game *Game) Init(width int, height int, title string) error {
	var err error
	err = glfw.Init()
	if err != nil {
		return err
	}
	glfw.WindowHint(glfw.DoubleBuffer, 1)
	game.Window, err = glfw.CreateWindow(width, height, title, nil, nil)
	if err != nil {
		return err
	}
	game.Window.MakeContextCurrent()
	glfw.SwapInterval(1)
	return nil
}
func (game *Game) Exit() {
	game.Window.Destroy()
	glfw.Terminate()
}
func (game *Game) Run() error {
	cont := true
	frameTime := float64(1.0 / game.World.Fps())
	last := glfw.GetTime()
	next := last + frameTime
	for cont {
		now := glfw.GetTime()
		diff := now - last
		last = now
		c, err := game.World.Move(float32(diff))
		log.Printf("diff: %f", diff*1000)
		if err != nil {
			return err
		}
		glfw.PollEvents()
		cont = cont && c && !game.Window.ShouldClose()
		wait := next - glfw.GetTime()
		if wait > 0 {
			err = game.World.Render(game.drawContext)
			if err != nil {
				return err
			}
			game.Window.SwapBuffers()
			delay := (next - glfw.GetTime()) * 1000 * 1000 * 1000
			time.Sleep( time.Duration(delay) )
		}
		next = next + frameTime
	}
	return nil
}
