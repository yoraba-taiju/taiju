#pragma once

#include "cocos2d.h"
#include "../Util.h"

class GameLayer : public cocos2d::LayerColor {
private:
	static const constexpr int TAG_PLAYER=1;
private:
	cocos2d::EventListener* touchListener_;
public:
	GameLayer();
	bool init(cocos2d::Size screenSize);
	virtual void onEnter() override;
	virtual void onExit() override;
	void onGameover();
	bool onTouchBegan(cocos2d::Touch* touch,cocos2d::Event* event);
	void onTouchMoved(cocos2d::Touch* touch, cocos2d::Event* event);
	void onTouchEnded(cocos2d::Touch* touch, cocos2d::Event* event);
	CREATE_FUNC_WITH_ARG(GameLayer, cocos2d::Size);
};
