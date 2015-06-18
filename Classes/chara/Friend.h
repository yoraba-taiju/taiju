#pragma once

#include "cocos2d.h"

class Friend : public cocos2d::Layer {
private:
	int life_;
public:
	int life() const { return this->life_; };
	void life(int life) { this->life_ = life; };
public:
	Friend();
	virtual bool init() override;
public:
	virtual void OnVanished() = 0;
};

class TeamMate : public Friend {
public:
	TeamMate();
	virtual bool init() override;
};
