#pragma once

#include "cocos2d.h"
#include "../Util.h"

class GameLayer : public cocos2d::Layer {
private:
	static const constexpr int TAG_PLAYER=1;
	cocos2d::Size fieldSize;
public:
	cocos2d::Size getFieldSize() const { return this->fieldSize; };
public:
	GameLayer();
	bool init(cocos2d::Size screenSize);
	bool onTouchBegan(cocos2d::Touch* touch,cocos2d::Event* event);
	void onTouchMoved(cocos2d::Touch* touch, cocos2d::Event* event);
	void onTouchEnded(cocos2d::Touch* touch, cocos2d::Event* event);
	CREATE_FUNC_WITH_ARG(GameLayer, cocos2d::Size);
};
