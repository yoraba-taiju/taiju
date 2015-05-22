package engine

import (

)

//Sceneを動かすための舞台装置
//

type World interface {
	Run(DrawContext)
	ChangeScene(Scene)
}

type WorldBase struct{
	
}

func (base *WorldBase) Run(ctx DrawContext){
	
}
func (base *WorldBase) ChangeScene(scene Scene){
	
}
	