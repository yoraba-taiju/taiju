#pragma once
#include "cocos2d.h"
#include "../Util.h"

using namespace cocos2d;

class BulletGenerator : public cocos2d::Node
{
	DEFINE_MEMBER(public, public, float, firePerMillisec);
	DEFINE_MEMBER(public, public, Vec2, angle);
private:
	bool enabled_;
	float left_ = 0;
public:
	virtual bool init() override;
	bool init(float firePerMillisec);
	virtual void update(float delta) override;
	virtual void fire() = 0;
	void setGenerationEnabled(bool enabled);
};

class NWayShotGenerator : public BulletGenerator
{
	DEFINE_MEMBER(public, public, int, n);
	DEFINE_MEMBER(public, public, float, range);
public:
	bool init(float const fpms, float const range, int const n);
	virtual void fire() override;
	CREATE_FUNC3(NWayShotGenerator, float, float, int);
};
