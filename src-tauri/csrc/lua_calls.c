#ifdef _WIN32
#define EXPORT __declspec(dllexport)
#else
#define EXPORT
#endif

#include <stdlib.h>

//Globals
EXPORT const char * subsystem() {
    #ifdef _WIN32
        return "Windows";
    #elif __APPLE__
        return "MacOS";
    #endif

    return "Linux";
}

//Functions
EXPORT int run_command(const char * arg) {
    return system(arg);
}
