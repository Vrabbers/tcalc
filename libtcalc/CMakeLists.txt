project(libtcalc VERSION 0.0.1 LANGUAGES CXX)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")
find_package(UTF8PROC REQUIRED)
find_package(GMP REQUIRED)
find_package(MPFR REQUIRED)
find_package(MPC REQUIRED)

set(SOURCES
    string_reader.cpp
    lexer.cpp
    token.cpp
    diagnostic.cpp
    parser.cpp
    expression.cpp
)

set(HEADERS
    source_span.h
    token.h
    lexer.h
    string_reader.h
    diagnostic.h
    parser.h
    expression.h
    number.h
)

set(PRIVATE_HEADERS
    utf8_utils.h
)

add_library(libtcalc STATIC ${SOURCES} ${HEADERS} ${PRIVATE_HEADERS})

set_target_properties(libtcalc PROPERTIES
    OUTPUT_NAME tcalc
)

target_include_directories(libtcalc PUBLIC ${UTF8PROC_INCLUDE_DIRS} ${GMP_INCLUDE_DIRS} ${MPFR_INCLUDE_DIRS} ${MPC_INCLUDE_DIRS})
target_link_libraries(libtcalc PUBLIC ${UTF8PROC_LIBRARIES} ${GMP_LIBRARIES} ${MPFR_LIBRARIES} ${MPC_LIBRARIES})
target_compile_definitions(libtcalc PRIVATE UTF8PROC_STATIC TCALC_LIBRARY)
