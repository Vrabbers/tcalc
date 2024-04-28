include(FindPackageHandleStandardArgs)

find_library(MPC_LIBRARIES
    NAMES mpc
)

find_path(MPC_INCLUDES
    NAMES mpc.h
)

find_package_handle_standard_args(MPC
    REQUIRED_VARS MPC_LIBRARIES MPC_INCLUDES)

if (MPC_FOUND AND NOT TARGET MPC::MPC)
    add_library(MPC::MPC UNKNOWN IMPORTED)
    set_target_properties(MPC::MPC PROPERTIES
        INTERFACE_INCLUDE_DIRECTORIES "${MPC_INCLUDES}"
        IMPORTED_LOCATION "${MPC_LIBRARIES}")
endif()