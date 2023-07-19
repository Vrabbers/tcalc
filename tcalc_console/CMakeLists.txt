project(tcalc VERSION 0.0.1 LANGUAGES CXX)

set(SOURCES 
    main.cpp)

add_executable(tcalc ${SOURCES} ${HEADERS})

target_link_libraries(tcalc PRIVATE libtcalc)
target_include_directories(tcalc PUBLIC ../libtcalc/)
