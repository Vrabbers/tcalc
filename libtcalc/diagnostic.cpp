#include "diagnostic.h"

#include <array>

const char* tcDiagnosticTypeName(tcDiagnosticType type)
{
    static std::array names =
    {
        "BadNumberLiteral",
        "BadCharacter",
        "BadSymbol"
    };

    return names.at(static_cast<std::size_t>(type));
}
