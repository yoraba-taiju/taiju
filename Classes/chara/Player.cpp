
#include "../core/GameLayer.h"
#include "Player.h"
using namespace cocos2d;

Player::Player(){
	
}

bool Player::init() {
	Layer::init();
	auto spr = Sprite::create("CloseNormal.png");
	this->addChild(spr);
	this->sprite_ = spr;
	
	return true;
}

void Player::move(cocos2d::Vec2 const& delta){
	auto parent = static_cast<GameLayer*>(this->getParent());
	auto fSize = parent->getContentSize();
	auto next = this->getPosition() + delta - (fSize/2);
	auto bbox = this->sprite_->getBoundingBox();
	auto size = (fSize - bbox.size)/2;
	
	if(next.x < -size.width){
		next.x = -size.width;
	}else if(next.x > size.width){
		next.x = size.width;
	}
	if(next.y < -size.height){
		next.y = -size.height;
	}else if(next.y > size.height){
		next.y = size.height;
	}
	this->setPosition(next + (fSize/2));
}


