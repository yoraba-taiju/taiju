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
	
	game := engine.NewGame(g.NewWorld())
	game.Run()
}
