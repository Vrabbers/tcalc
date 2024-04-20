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
    source_span.cpp
    utf8_utils.cpp
    token.cpp
    lexer.cpp
)
    
set(HEADERS
    source_position.h
    tcalc_export.h
    source_span.h
    token.h
    lexer.h
    token_type.h
    string_reader.h
)

set(PRIVATE_HEADERS
    utf8_utils.h
)

add_library(libtcalc SHARED ${SOURCES} ${HEADERS} ${PRIVATE_HEADERS} "token_type.h")
set_target_properties(libtcalc PROPERTIES 
    OUTPUT_NAME tcalc
)

target_include_directories(libtcalc PRIVATE ${LIBUTF8PROC_INCLUDE_DIRS})
target_link_directories(libtcalc PRIVATE ${LIBUTF8PROC_LIBRARY_DIRS})
target_link_libraries(libtcalc PRIVATE ${LIBUTF8PROC_LIBRARIES})
target_compile_definitions(libtcalc PRIVATE UTF8PROC_STATIC TCALC_LIBRARY)
