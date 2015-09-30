#include "BulletGenerator.h"

bool BulletGenerator::init() {
	this->Node::init();
	this->scheduleUpdate();
	return true;
}
bool BulletGenerator::init(float firePerMillisec){
	this->firePerMillisec_ = firePerMillisec;
	this->left_ = 0;
	this->enabled_ = true;
	return this->init();
}

void BulletGenerator::update(float delta) {
	if(!this->enabled_){
		return;
	}
	this->left_ -= delta;
	if(this->left_ < 0){
		this->left_ += this->firePerMillisec_;
		this->fire();
	}
}

void BulletGenerator::setGenerationEnabled(bool enabled){
	if(enabled != this->enabled_){
		this->enabled_ = enabled;
		if (enabled) {
			this->scheduleUpdate();
		}else{
			this->unscheduleUpdate();
		}
	}
}


bool NWayShotGenerator::init(float const fpms, float const range,int const n) {
	this->n(n);
	this->range(range);
	return this->BulletGenerator::init(fpms);
}
void NWayShotGenerator::fire() {
	std::vector<Vec2> directions;
	Vec2 from = this->angle().rotateByAngle(Vec2::ZERO, -this->range()/2);
	float delta = this->range()/n_;
	for(int i=0;i<this->n_;++i){
		Vec2 angle(from.rotateByAngle(Vec2::ZERO, delta*i));
		this->addChild();
	}
}
