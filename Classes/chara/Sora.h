#pragma once

#include "cocos2d.h"
#include "Friend.h"

class Sora : public Friend {
private:
	cocos2d::Sprite* sprite_;
public:
	Sora();
	virtual bool init() override;
	void move(cocos2d::Vec2 const& delta);
	CREATE_FUNC(Sora);
public:
	virtual void OnVanished() override;
};
