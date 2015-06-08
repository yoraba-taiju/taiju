package msg

import ()

type DamageMessage float32

func NewDamageMessage(value float32) DamageMessage {
	return DamageMessage(value)
}
