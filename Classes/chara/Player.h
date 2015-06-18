#pragma once

#include "cocos2d.h"
#include "../Util.h"

class Player : public cocos2d::Layer {
private:
	cocos2d::Sprite* sprite_;
public:
	Player();
	virtual bool init() override;
	void move(cocos2d::Vec2 const& delta);
	CREATE_FUNC(Player);
};
