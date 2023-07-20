#pragma once

#pragma warning (disable: 4251)
// Please make sure you're building this library with a compatible ABI with the rest of your program!

#ifdef _WIN32
    #ifdef TCALC_LIBRARY
        #define TCALC_EXPORT __declspec(dllexport)
    #else
        #define TCALC_EXPORT __declspec(dllimport)
    #endif
#else 
    #define TCALC_EXPORT
#endif

