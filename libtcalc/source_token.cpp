#include "source_token.h"

TCalc::SourceToken::SourceToken(std::string str, SourcePosition st, SourcePosition e)
{
    string = std::move(str);
    start = st;
    end = e;
}

const std::string& TCalc::SourceToken::getString() const
{
    return string;
}

TCalc::SourcePosition TCalc::SourceToken::getStart() const
{
    return start;
}

TCalc::SourcePosition TCalc::SourceToken::getEnd() const
{
    return end;
}