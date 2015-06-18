#pragma once

#define CREATE_FUNC_WITH_ARG(__TYPE__, __EXT__) \
static __TYPE__* create(__EXT__ __arg__) \
{ \
__TYPE__ *pRet = new(std::nothrow) __TYPE__(); \
if (pRet && pRet->init(__arg__)) \
{ \
pRet->autorelease(); \
return pRet; \
} \
else \
{ \
delete pRet; \
pRet = NULL; \
return NULL; \
} \
}
