include(FindPackageHandleStandardArgs)

find_library(GMP_LIBRARIES
    NAMES gmp
)

find_path(GMP_INCLUDES
    NAMES gmp.h
)

find_package_handle_standard_args(GMP
    REQUIRED_VARS GMP_LIBRARIES GMP_INCLUDES)

if (GMP_FOUND AND NOT TARGET GMP::GMP)
    add_library(GMP::GMP UNKNOWN IMPORTED)
    set_target_properties(GMP::GMP PROPERTIES
        INTERFACE_INCLUDE_DIRECTORIES "${GMP_INCLUDES}"
        IMPORTED_LOCATION "${GMP_LIBRARIES}")
endif()