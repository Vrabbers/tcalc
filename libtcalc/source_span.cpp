#include "source_span.h"

tcSourceSpan::tcSourceSpan(std::string&& str, tcSourcePosition start) : _string(str), _position(start)
{ }

std::string_view tcSourceSpan::string() const
{
    return _string;
}

tcSourcePosition tcSourceSpan::position() const
{
    return _position;
}