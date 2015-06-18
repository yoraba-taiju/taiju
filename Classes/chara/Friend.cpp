
#include "../core/GameLayer.h"
#include "Friend.h"
using namespace cocos2d;

Friend::Friend(){
	
}

bool Friend::init() {
	Layer::init();
	return true;
}

TeamMate::TeamMate(){
	
}

bool TeamMate::init() {
	Friend::init();
	return true;
}

