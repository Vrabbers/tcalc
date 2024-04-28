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

if(NOT DEFINED GMP_ROOT)
    message(FATAL_ERROR "Please set GMP_ROOT to locate the GNU GMP library.")
else()
    set(GMP_INCLUDE_DIRS "${GMP_ROOT}/include")
    set(GMP_LIBRARY_DIRS "${GMP_ROOT}/lib")
    set(GMP_LIBRARIES "${GMP_ROOT}/lib/gmp.lib")
endif()

if(NOT DEFINED MPFR_ROOT)
    message(FATAL_ERROR "Please set MPFR_ROOT to locate the MPFR library.")
else()
    set(MPFR_INCLUDE_DIRS "${MPFR_ROOT}/include")
    set(MPFR_LIBRARY_DIRS "${MPFR_ROOT}/lib")
    set(MPFR_LIBRARIES "${MPFR_ROOT}/lib/mpfr.lib")
endif()

if(NOT DEFINED MPC_ROOT)
    message(FATAL_ERROR "Please set MPC_ROOT to locate the MPC library.")
else()
    set(MPC_INCLUDE_DIRS "${MPC_ROOT}/include")
    set(MPC_LIBRARY_DIRS "${MPC_ROOT}/lib")
    set(MPC_LIBRARIES "${MPC_ROOT}/lib/mpc.lib")
endif()


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

target_include_directories(libtcalc PUBLIC ${LIBUTF8PROC_INCLUDE_DIRS} ${GMP_INCLUDE_DIRS} ${MPFR_INCLUDE_DIRS} ${MPC_INCLUDE_DIRS})
target_link_directories(libtcalc PUBLIC ${LIBUTF8PROC_LIBRARY_DIRS} ${GMP_LIBRARY_DIRS} ${MPFR_LIBRARY_DIRS} ${MPC_LIBRARY_DIRS})
target_link_libraries(libtcalc PUBLIC ${LIBUTF8PROC_LIBRARIES} ${GMP_LIBRARIES} ${MPFR_LIBRARIES} ${MPC_LIBRARIES})
target_compile_definitions(libtcalc PRIVATE UTF8PROC_STATIC TCALC_LIBRARY)
