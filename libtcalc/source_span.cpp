#include "source_span.h"

SourceSpan::SourceSpan(std::string&& str, SourcePosition st) : _string(str), _position(st)
{ }

const std::string& SourceSpan::string() const
{
    return _string;
}

SourcePosition SourceSpan::position() const
{
    return _position;
}
