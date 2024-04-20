#include "source_span.h"

tcSourceSpan::tcSourceSpan(std::string&& str, tcSourcePosition st) : _string(str), _position(st)
{ }

const char* tcSourceSpan::string() const
{
    return _string.c_str();
}

tcSourcePosition tcSourceSpan::position() const
{
    return _position;
}

tcSourceSpan::~tcSourceSpan() = default;
