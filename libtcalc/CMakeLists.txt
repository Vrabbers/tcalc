project(libtcalc VERSION 0.0.1 LANGUAGES CXX)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")
find_package(UTF8PROC REQUIRED)
find_package(GMP REQUIRED)
find_package(MPFR REQUIRED)
find_package(MPC REQUIRED)

set(SOURCES
    tc_string_reader.cpp
    tc_lexer.cpp
    tc_token.cpp
    tc_diagnostic.cpp
    tc_parser.cpp
    tc_number.cpp
    tc_operation.cpp
    utf8utils.cpp
    tc_evaluator.cpp
    tc_eval_result.cpp
)

set(HEADERS
    tc_source_position.h
    tc_token.h
    tc_lexer.h
    tc_string_reader.h
    tc_diagnostic.h
    tc_parser.h
    tc_expression.h
    tc_number.h
    tc_operation.h
    tc_evaluator.h
    tc_eval_result.h
)


add_library(libtcalc STATIC ${SOURCES} ${HEADERS})

set_target_properties(libtcalc PROPERTIES
    OUTPUT_NAME tcalc
    PUBLIC_HEADER "${HEADERS}"
)

target_include_directories(libtcalc PUBLIC ${UTF8PROC_INCLUDES} ${GMP_INCLUDES} ${MPFR_INCLUDES} ${MPC_INCLUDES})
target_link_libraries(libtcalc PUBLIC ${UTF8PROC_LIBRARIES} ${GMP_LIBRARIES} ${MPFR_LIBRARIES} ${MPC_LIBRARIES})

include(GNUInstallDirs)
include(CMakePackageConfigHelpers)

install(TARGETS libtcalc
    EXPORT libtcalcTargets
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/libtcalc
)

set(CMAKECONFIG_INSTALL_DIR ${CMAKE_INSTALL_LIBDIR}/cmake/libtcalc)
configure_package_config_file(libtcalcConfig.cmake.in ${CMAKE_CURRENT_BINARY_DIR}/libtcalcConfig.cmake
        INSTALL_DESTINATION ${CMAKECONFIG_INSTALL_DIR})

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/libtcalcConfig.cmake
        DESTINATION ${CMAKECONFIG_INSTALL_DIR})
install(EXPORT libtcalcTargets
        FILE libtcalcTargets.cmake
        DESTINATION ${CMAKECONFIG_INSTALL_DIR})
