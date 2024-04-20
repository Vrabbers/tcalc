#include "utf8_utils.h"

#include "utf8proc.h"

std::string tcEncodeCodepoint(char32_t character)
{
    char buf[4] = { };
    const auto count = utf8proc_encode_char(static_cast<int32_t>(character), reinterpret_cast<uint8_t*>(buf));
    return {buf, static_cast<std::size_t>(count)};
}
