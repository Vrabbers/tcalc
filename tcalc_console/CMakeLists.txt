project(tcalc VERSION 0.0.1 LANGUAGES CXX)

set(SOURCES 
    main.cpp)

add_executable(tcalc ${SOURCES} ${HEADERS})

target_link_libraries(tcalc PRIVATE libtcalc)
target_include_directories(tcalc PUBLIC ../libtcalc/)

install(FILES $<TARGET_RUNTIME_DLLS:tcalc> TYPE BIN)

add_custom_command(TARGET tcalc POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy -t $<TARGET_FILE_DIR:tcalc> $<TARGET_RUNTIME_DLLS:tcalc>
    COMMAND_EXPAND_LISTS)