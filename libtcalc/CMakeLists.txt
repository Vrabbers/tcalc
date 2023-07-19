project(libtcalc VERSION 0.0.1 LANGUAGES CXX)


if(NOT (DEFINED LIBUTF8PROC_INCLUDE_DIRS AND DEFINED LIBUTF8PROC_LIBRARY_DIRS AND DEFINED LIBUTF8PROC_LIBRARIES))
    if(${CMAKE_SYSTEM_NAME} MATCHES "Windows")
        set(LIBUTF8PROC_INCLUDE_DIRS "C:/Program Files (x86)/utf8proc/include")
        set(LIBUTF8PROC_LIBRARY_DIRS "C:/Program Files (x86)/utf8proc/lib")
        set(LIBUTF8PROC_LIBRARIES "C:/Program Files (x86)/utf8proc/lib/utf8proc_static.lib")
    else()
        find_package(PkgConfig)
        if (PkgConfig_FOUND)
            pkg_check_modules(LIBUTF8PROC libutf8proc)
        endif()
    endif()
endif()

set(SOURCES 
    string_reader.cpp
)
    
set(HEADERS
    source_position.h
    string_reader.h
    tcalc_export.h
)

set(PRIVATE_HEADERS
)

add_library(libtcalc SHARED ${SOURCES} ${HEADERS} ${PRIVATE_HEADERS})
set_target_properties(libtcalc PROPERTIES 
    OUTPUT_NAME tcalc
)

target_include_directories(libtcalc PRIVATE ${LIBUTF8PROC_INCLUDE_DIRS})
target_link_directories(libtcalc PRIVATE ${LIBUTF8PROC_LIBRARY_DIRS})
target_link_libraries(libtcalc PRIVATE ${LIBUTF8PROC_LIBRARIES})
target_compile_definitions(libtcalc PRIVATE UTF8PROC_STATIC TCALC_LIBRARY)
