#include "source_span.h"

SourceSpan::SourceSpan(std::string_view str, SourcePosition st) : _string(str), _position(st)
{ }

std::string_view SourceSpan::string() const
{
    return _string;
}

SourcePosition SourceSpan::position() const
{
    return _position;
}
