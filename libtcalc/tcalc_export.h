#pragma once

#ifdef _WIN32
    #ifdef TCALC_LIBRARY
        #define TCALC_EXPORT __declspec(dllexport)
    #else
        #define TCALC_EXPORT __declspec(dllimport)
    #endif
#else 
    #define TCALC_EXPORT
#endif

