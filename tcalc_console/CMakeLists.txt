project(tcalc VERSION 0.0.1 LANGUAGES CXX)

set(SOURCES 
    main.cpp)

add_executable(tcalc ${SOURCES} ${HEADERS})

target_link_libraries(tcalc PRIVATE libtcalc)
target_include_directories(tcalc PUBLIC ../libtcalc/)

install(FILES $<TARGET_RUNTIME_DLLS:tcalc> TYPE BIN)
install(TARGETS tcalc DESTINATION bin)