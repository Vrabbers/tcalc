#include "source_span.h"

tcSourceSpan::tcSourceSpan(std::string&& str, SourcePosition st) : _string(str), _position(st)
{ }

const char* tcSourceSpan::string() const
{
    return _string.c_str();
}

SourcePosition tcSourceSpan::position() const
{
    return _position;
}
