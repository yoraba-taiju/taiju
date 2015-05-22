package engine

import (

)

type Matrix struct {
	values [16]float32
}

func (m *Matrix) mult(o *Matrix) *Matrix{
	
	return m;
}

type Vector struct{
	X float32
	Y float32
}
type Point Vector;

func (p *Point) Add(v Vector) *Point {
	p.X += v.X;
	p.Y += v.Y;
	return p;
}

func IsHitBox(mp Point, ms Vector, op Point, os Vector) bool{
	mx1 := mp.X - os.X/2
	mx2 := mp.X + os.X/2
	my1 := mp.Y - os.Y/2
	my2 := mp.Y + os.Y/2
	ex1 := op.X - os.X/2
	ex2 := op.X + os.X/2
	ey1 := op.Y - os.Y/2
	ey2 := op.Y + os.Y/2
	return mx1 <= ex2 && ex1 <= mx2 && my1 <= ey2 && ey1 <= my2
}