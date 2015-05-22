package engine

import (

)

type Affine interface {
}

type DrawContext interface{
	Draw(Point,Drawable)
	Trans(Affine) DrawContext
}

type Drawable interface {
}
