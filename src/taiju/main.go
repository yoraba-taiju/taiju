package main

import (
	"flag"
	"log"
	"taiju/engine"
	g "taiju/game"
)

var versionFlag *bool = flag.Bool("v", false, "be verbose")

func main() {
	flag.Parse()
	log.Printf("----------------------------------------")
	log.Printf("              Yoraba Taiju")
	log.Printf("----------------------------------------")

	var err error
	game := engine.NewGame(g.NewWorld())
	err = game.Init(800, 600, "Yoraba Taiju")
	if err != nil {
		panic(err)
	}
	defer game.Exit()
	err = game.Run()
	if err != nil {
		panic(err)
	}
	log.Printf("Successfully Closed.")
}
