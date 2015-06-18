#include "GameLayer.h"
#include "../chara/Player.h"

using namespace cocos2d;

GameLayer::GameLayer(){
	
}

bool GameLayer::init(cocos2d::Size fieldSize) {
	this->LayerColor::init();
	this->setColor(cocos2d::Color3B::GRAY);
	this->setOpacity(255);
	auto player = Player::create();
	player->setTag(TAG_PLAYER);
	this->addChild(player);
	this->setContentSize(fieldSize);
	player->setPosition(fieldSize/2);
	
	auto listener = EventListenerTouchOneByOne::create();
	listener->setSwallowTouches(true);
	
	listener->onTouchBegan = CC_CALLBACK_2(GameLayer::onTouchBegan, this);
	listener->onTouchMoved = CC_CALLBACK_2(GameLayer::onTouchMoved, this);
	listener->onTouchEnded = CC_CALLBACK_2(GameLayer::onTouchEnded, this);
	
	auto disp = Director::getInstance()->getEventDispatcher();
	disp->addEventListenerWithSceneGraphPriority(listener, this);

	return true;
}


bool GameLayer::onTouchBegan(cocos2d::Touch* touch,cocos2d::Event* event) {
	return true;
}
void GameLayer::onTouchMoved(cocos2d::Touch* touch, cocos2d::Event* event) {
	Player* player = static_cast<Player*>(this->getChildByTag(TAG_PLAYER));
	player->move(touch->getDelta());
}
void GameLayer::onTouchEnded(cocos2d::Touch* touch, cocos2d::Event* event) {
	
}
