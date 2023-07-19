project(libtcalc VERSION 0.0.1 LANGUAGES CXX)

find_package(PkgConfig)
if (PkgConfig_FOUND)
    pkg_check_modules(LIBUTF8PROC libutf8proc)
endif()

set(SOURCES 
    string_reader.cpp
)

set(HEADERS
    source_position.h
    string_reader.h
)

set(PRIVATE_HEADERS
)

add_library(libtcalc SHARED ${SOURCES} ${HEADERS} ${PRIVATE_HEADERS})
set_target_properties(libtcalc PROPERTIES OUTPUT_NAME tcalc)


target_include_directories(libtcalc PRIVATE ${LIBUTF8PROC_INCLUDE_DIRS})
target_link_directories(libtcalc PRIVATE ${LIBUTF8PROC_LIBRARY_DIRS})
target_link_libraries(libtcalc PRIVATE ${LIBUTF8PROC_LIBRARIES})