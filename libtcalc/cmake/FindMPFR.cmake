include(FindPackageHandleStandardArgs)

find_library(MPFR_LIBRARIES
    NAMES mpfr
)

find_path(MPFR_INCLUDES
    NAMES mpfr.h
)

find_package_handle_standard_args(MPFR
    REQUIRED_VARS MPFR_LIBRARIES MPFR_INCLUDES)

if (MPFR_FOUND AND NOT TARGET MPFR::MPFR)
    add_library(MPFR::MPFR UNKNOWN IMPORTED)
    set_target_properties(MPFR::MPFR PROPERTIES
        INTERFACE_INCLUDE_DIRECTORIES "${MPFR_INCLUDES}"
        IMPORTED_LOCATION "${MPFR_LIBRARIES}")
endif()