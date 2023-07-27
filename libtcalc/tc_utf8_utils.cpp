#include "tc_utf8_utils.h"

#include "utf8proc.h"

std::string tcEncodeCodepoint(char32_t character)
{
    char buf[4];
    utf8proc_encode_char(character, reinterpret_cast<uint8_t*>(buf));
    return std::string(buf);
}