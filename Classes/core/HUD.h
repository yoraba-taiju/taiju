#pragma once

#include "cocos2d.h"
#include "../Util.h"

class ScoreLayer : public cocos2d::LayerColor {
public:
	ScoreLayer();
	virtual bool init() override;
	CREATE_FUNC(ScoreLayer);
};

class GameoverLayer : public cocos2d::LayerColor {
public:
	GameoverLayer();
	virtual bool init() override;
	CREATE_FUNC(GameoverLayer);
};
