#pragma once

#define CREATE_FUNC1(__TYPE__, __EXT__) \
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

#define CREATE_FUNC2(__TYPE__, __EXT__, __EXT2__) \
static __TYPE__* create(__EXT__ __arg__, __EXT2__ __arg2__) \
{ \
__TYPE__ *pRet = new(std::nothrow) __TYPE__(); \
if (pRet && pRet->init(__arg__, __arg2__)) \
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

#define CREATE_FUNC3(__TYPE__, __EXT__, __EXT2__, __EXT3__) \
static __TYPE__* create(__EXT__ __arg__, __EXT2__ __arg2__, __EXT2__ __arg3__) \
{ \
__TYPE__ *pRet = new(std::nothrow) __TYPE__(); \
if (pRet && pRet->init(__arg__, __arg2__, __arg3__)) \
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

#define DEFINE_MEMBER_CONST(rscope, type, name)\
private:\
type const name##_;\
rscope:\
inline type const& name() const noexcept{return name##_;}

#define DEFINE_MEMBER_REF(rscope, type, name)\
private:\
type& name##_;\
rscope:\
inline type& name() const noexcept{return name##_;}

#define DEFINE_MEMBER(rscope, wscope, type, name)\
private:\
type name##_;\
rscope:\
inline type const& name() const noexcept{return name##_;}\
wscope:\
inline void name(type const& val){name##_ = val;}

#define DEFINE_CONTAINER(rscope, wscope, ctype, name)\
private:\
ctype name##_;\
rscope:\
inline ctype const& name() const noexcept{return name##_;}\
wscope:\
inline ctype& name() noexcept{return name##_;}\
inline void name(ctype const& val){name##_ = val;}

#define DEFINE_MEMBER_LITERAL(rscope, wscope, type, name)\
private:\
type name##_;\
rscope:\
inline constexpr type const& name() const noexcept{return name##_;}\
wscope:\
inline void name(type const& val){name##_ = val;}