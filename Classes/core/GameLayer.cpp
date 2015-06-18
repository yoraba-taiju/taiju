#include "GameLayer.h"
#include "../chara/Sora.h"

using namespace cocos2d;

GameLayer::GameLayer(){
	
}

bool GameLayer::init(cocos2d::Size fieldSize) {
	this->LayerColor::init();
	this->setColor(cocos2d::Color3B::GRAY);
	this->setOpacity(255);
	auto player = Sora::create();
	player->setTag(TAG_PLAYER);
	this->addChild(player);
	this->setContentSize(fieldSize);
	player->setPosition(fieldSize/2);
	
	return true;
}

void GameLayer::onEnter() {
	this->LayerColor::onEnter();
	auto listener = EventListenerTouchOneByOne::create();
	listener->setSwallowTouches(true);
	
	listener->onTouchBegan = CC_CALLBACK_2(GameLayer::onTouchBegan, this);
	listener->onTouchMoved = CC_CALLBACK_2(GameLayer::onTouchMoved, this);
	listener->onTouchEnded = CC_CALLBACK_2(GameLayer::onTouchEnded, this);
	
	auto disp = Director::getInstance()->getEventDispatcher();
	disp->addEventListenerWithSceneGraphPriority(listener, this);
	this->touchListener_ = listener;
}
void GameLayer::onExit() {
	this->LayerColor::onExit();
	auto disp = Director::getInstance()->getEventDispatcher();
	disp->removeEventListener(this->touchListener_);
	this->touchListener_ = nullptr;
}


void GameLayer::onGameover(){
}

bool GameLayer::onTouchBegan(cocos2d::Touch* touch,cocos2d::Event* event) {
	return true;
}
void GameLayer::onTouchMoved(cocos2d::Touch* touch, cocos2d::Event* event) {
	Sora* sora = static_cast<Sora*>(this->getChildByTag(TAG_PLAYER));
	sora->move(touch->getDelta());
}
void GameLayer::onTouchEnded(cocos2d::Touch* touch, cocos2d::Event* event) {
	
}
