project(libtcalc VERSION 0.0.1 LANGUAGES CXX)

set(SOURCES 
    lib.cpp
)

set(HEADERS
    lib.h
)

add_library(libtcalc SHARED ${SOURCES} ${HEADERS} ${PRIVATE_HEADERS})