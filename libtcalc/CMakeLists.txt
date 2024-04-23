project(libtcalc VERSION 0.0.1 LANGUAGES CXX)

find_package(PkgConfig)
if(NOT DEFINED LIBUTF8PROC_ROOT AND NOT PkgConfig_FOUND)
    message(FATAL_ERROR "Please set LIBUTF8PROC_ROOT to locate the utf8proc library.")
elseif(DEFINED LIBUTF8PROC_ROOT)
    set(LIBUTF8PROC_INCLUDE_DIRS "${LIBUTF8PROC_ROOT}/include")
    set(LIBUTF8PROC_LIBRARY_DIRS "${LIBUTF8PROC_ROOT}/lib")
    set(LIBUTF8PROC_LIBRARIES "${LIBUTF8PROC_ROOT}/lib/utf8proc_static.lib")
elseif(PkgConfig_FOUND)
    pkg_check_modules(LIBUTF8PROC libutf8proc)
endif()

set(SOURCES 
    string_reader.cpp
    lexer.cpp
    token.cpp
    diagnostic.cpp
)

set(HEADERS
    source_position.h
    source_span.h
    token.h
    lexer.h
    token_kind.h
    string_reader.h
    diagnostic.h
)

set(PRIVATE_HEADERS
    utf8_utils.h
)

add_library(libtcalc STATIC ${SOURCES} ${HEADERS} ${PRIVATE_HEADERS})

set_target_properties(libtcalc PROPERTIES 
    OUTPUT_NAME tcalc
)

target_include_directories(libtcalc PRIVATE ${LIBUTF8PROC_INCLUDE_DIRS})
target_link_directories(libtcalc PRIVATE ${LIBUTF8PROC_LIBRARY_DIRS})
target_link_libraries(libtcalc PRIVATE ${LIBUTF8PROC_LIBRARIES})
target_compile_definitions(libtcalc PRIVATE UTF8PROC_STATIC TCALC_LIBRARY)
