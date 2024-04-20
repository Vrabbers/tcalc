#include "lexer.h"

#include <iostream>

#include "utf8proc.h"
#include "utf8_utils.h"

static bool isWhitespace(std::optional<char32_t> chr)
{
    if (!chr.has_value())
        return false;
    // tab is not in category ZS. conveniently, neither is LF.
    return chr == U'\t' || utf8proc_category(static_cast<std::int32_t>(chr.value())) == UTF8PROC_CATEGORY_ZS;
}

static bool isLetter(std::optional<char32_t> chr)
{
    if (!chr.has_value())
        return false;
    auto c = utf8proc_category(static_cast<std::int32_t>(chr.value()));
    return (c >= UTF8PROC_CATEGORY_LU && c <= UTF8PROC_CATEGORY_LO) || c == UTF8PROC_CATEGORY_PC;
}

static bool isDigit(std::optional<char32_t> chr)
{
    return chr >= U'0' && chr <= U'9';
}

static bool isHexDigit(std::optional<char32_t> chr)
{
    return isDigit(chr) || (chr >= U'a' && chr <= U'f') || (chr >= U'A' && chr <= U'F');
}

tcLexer::tcLexer(std::string&& input, bool commaArgSeparator)
    : _sr(std::make_unique<tcStringReader>(std::move(input))), _commaArgumentSeparator(commaArgSeparator)
{
}

tcLexer::tcLexer(const char* input, bool commaArgSeparator)
    :_sr(std::make_unique<tcStringReader>(input)), _commaArgumentSeparator(commaArgSeparator)
{

}

tcToken tcLexer::next()
{
    while (isWhitespace(_sr->peekNextCharacter()))
    {
        _sr->moveNextCharacter();
    }
    _sr->discardToken();

    const auto chOpt = _sr->moveNextCharacter();

    if (!chOpt.has_value())
        return flushToken(tcTokenType::Bad);
    
    auto first = chOpt.value();

    if (first == EndOfFile)
        return flushToken(tcTokenType::EndOfFile);
    if (first == U'\n') //TODO: nicer check here
        return flushToken(tcTokenType::EndOfLine);
    if (isDigit(first))
        return parseNumber(first);

    return parseSymbol(first);
}

tcLexer::~tcLexer() = default;

tcToken tcLexer::flushToken(tcTokenType type)
{
    auto span = _sr->flush();
    return {type, std::move(span)};
}

tcToken tcLexer::parseNumber(const char32_t first)
{
    auto next = _sr->peekNextCharacter();
    if (first == U'0' && next == U'b')
    {
        _sr->moveNextCharacter();
        while (_sr->peekNextCharacter() == U'0' || _sr->peekNextCharacter() == U'1' || _sr->peekNextCharacter() == U'_')
            _sr->moveNextCharacter();

        return flushToken(tcTokenType::BinaryLiteral);
    }

    if (first == U'0' && next == U'x')
    {
        _sr->moveNextCharacter();
        while (isHexDigit(_sr->peekNextCharacter()) || _sr->peekNextCharacter() == U'_')
            _sr->moveNextCharacter();

        return flushToken(tcTokenType::HexLiteral);
    }

    bool parsingAfterDecimal = false;
    bool parsingExponent = false;
    while (true)
    {
        next = _sr->peekNextCharacter();
        
        if (isDigit(next) || next == U'_')
        {
            _sr->moveNextCharacter();
        }
        else if (next == decimalSeparator())
        {
            _sr->moveNextCharacter();

            if (!parsingAfterDecimal)
                parsingAfterDecimal = true;
            else
                return flushToken(tcTokenType::Bad);
        }
        else if (next == U'e' || next == U'E')
        {
            _sr->moveNextCharacter();

            if (!parsingExponent)
            {
                parsingExponent = true;
                parsingAfterDecimal = true;
                next = _sr->peekNextCharacter();
                if (next == U'+' || next == U'-')
                    _sr->moveNextCharacter();
            }
            else
            {
                return flushToken(tcTokenType::Bad);
            }
        }
        else
        {
            if (_sr->peekNextCharacter() == U'i')
                _sr->moveNextCharacter();
            
            return flushToken(tcTokenType::NumericLiteral);
        }
    }
}

tcToken tcLexer::parseSymbol(char32_t first)
{
    std::optional<char32_t> next;
    switch (first)
    {
    case U'+':
        return flushToken(tcTokenType::Add);
    case U'⁺':
        return flushToken(tcTokenType::SuperscriptAdd);
    case U'-':
        return flushToken(tcTokenType::Subtract);
    case U'⁻':
        return flushToken(tcTokenType::SuperscriptSubtract);
    case U'*':
    case U'×':
    case U'∙':
        return flushToken(tcTokenType::Multiply);
    case U'/':
    case U'÷':
        return flushToken(tcTokenType::Divide);
    case U'^':
        return flushToken(tcTokenType::Exponentiate);
    case U'(':
        return flushToken(tcTokenType::LeftParens);
    case U')':
        return flushToken(tcTokenType::RightParens);
    case U'√':
        return flushToken(tcTokenType::Radical);
    case U'%':
        return flushToken(tcTokenType::Percent);
    case U'!':
        if (_sr->peekNextCharacter() == U'=')
        {
            _sr->moveNextCharacter();
            return flushToken(tcTokenType::NotEqual);
        }
        return flushToken(tcTokenType::Factorial);
    case U'>':
        next = _sr->peekNextCharacter();
        if (next == U'>')
        {
            _sr->moveNextCharacter();
            return flushToken(tcTokenType::RightShift);
        }
        if (next == U'=')
        {
            _sr->moveNextCharacter();
            return flushToken(tcTokenType::GreaterOrEqual);
        }
        return flushToken(tcTokenType::Greater);
    case U'<':
        next = _sr->peekNextCharacter();
        if (next == U'<')
        {
            _sr->moveNextCharacter();
            return flushToken(tcTokenType::LeftShift);
        }
        if (next == U'=')
        {
            _sr->moveNextCharacter();
            return flushToken(tcTokenType::LessOrEqual);
        }
        return flushToken(tcTokenType::Less);
    case U'≥':
        return flushToken(tcTokenType::GreaterOrEqual);
    case U'≤':
        return flushToken(tcTokenType::LessOrEqual);
    case U'=':
        if (_sr->peekNextCharacter() == U'=')
        {
            _sr->moveNextCharacter();
            return flushToken(tcTokenType::Equality);
        }
        return flushToken(tcTokenType::Equal);
    case U'≠':
        return flushToken(tcTokenType::NotEqual);
    case U'π':
        return flushToken(tcTokenType::Pi);
    case U'τ':
        return flushToken(tcTokenType::Tau);
    default:
        return flushToken(tcTokenType::Bad);
    }
}

char32_t tcLexer::decimalSeparator() const
{
    return _commaArgumentSeparator ? U'.' : U',';
}

char32_t tcLexer::argSeparator() const
{
    return _commaArgumentSeparator ? U',' : U';';
}