#include "lexer.h"

#include <unordered_map>

#include "utf8proc.h"

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
    while (isWhitespace(_sr->peek()))
    {
        _sr->forward();
    }
    _sr->discardToken();

    const auto chOpt = _sr->forward();

    if (!chOpt.has_value())
        return flushToken(tcTokenType::Bad);
    
    auto first = chOpt.value();

    if (first == EndOfFile)
        return flushToken(tcTokenType::EndOfFile);
    if (first == U'\n')
        return flushToken(tcTokenType::EndOfLine);
    if (isDigit(first))
        return parseNumber(first);
    if (isLetter(first))
        return parseIdentifier();

    return parseSymbol(first);
}

tcToken tcLexer::flushToken(tcTokenType type)
{
    auto span = _sr->flush();
    return {type, std::move(span)};
}

tcToken tcLexer::parseNumber(const char32_t first)
{
    auto next = _sr->peek();
    if (first == U'0' && next == U'b')
    {
        _sr->forward();
        while (_sr->peek() == U'0' || _sr->peek() == U'1' || _sr->peek() == U'_')
            _sr->forward();

        return flushToken(tcTokenType::BinaryLiteral);
    }

    if (first == U'0' && next == U'x')
    {
        _sr->forward();
        while (isHexDigit(_sr->peek()) || _sr->peek() == U'_')
            _sr->forward();

        return flushToken(tcTokenType::HexLiteral);
    }

    bool parsingAfterDecimal = false;
    bool parsingExponent = false;
    while (true)
    {
        next = _sr->peek();
        
        if (isDigit(next) || next == U'_')
        {
            _sr->forward();
        }
        else if (next == decimalSeparator())
        {
            _sr->forward();

            if (!parsingAfterDecimal)
                parsingAfterDecimal = true;
            else
                return flushToken(tcTokenType::Bad);
        }
        else if (next == U'e' || next == U'E')
        {
            _sr->forward();

            if (!parsingExponent)
            {
                parsingExponent = true;
                parsingAfterDecimal = true;
                next = _sr->peek();
                if (next == U'+' || next == U'-')
                    _sr->forward();
            }
            else
            {
                return flushToken(tcTokenType::Bad);
            }
        }
        else
        {
            if (_sr->peek() == U'i')
                _sr->forward();
            
            return flushToken(tcTokenType::NumericLiteral);
        }
    }
}

tcToken tcLexer::parseSymbol(char32_t first)
{
    if (first == argSeparator())
        return flushToken(tcTokenType::ArgumentSeparator);

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
        if (_sr->peek() == U'=')
        {
            _sr->forward();
            return flushToken(tcTokenType::NotEqual);
        }
        return flushToken(tcTokenType::Factorial);
    case U'>':
        next = _sr->peek();
        if (next == U'>')
        {
            _sr->forward();
            return flushToken(tcTokenType::RightShift);
        }
        if (next == U'=')
        {
            _sr->forward();
            return flushToken(tcTokenType::GreaterOrEqual);
        }
        return flushToken(tcTokenType::Greater);
    case U'<':
        next = _sr->peek();
        if (next == U'<')
        {
            _sr->forward();
            return flushToken(tcTokenType::LeftShift);
        }
        if (next == U'=')
        {
            _sr->forward();
            return flushToken(tcTokenType::LessOrEqual);
        }
        return flushToken(tcTokenType::Less);
    case U'≥':
        return flushToken(tcTokenType::GreaterOrEqual);
    case U'≤':
        return flushToken(tcTokenType::LessOrEqual);
    case U'=':
        if (_sr->peek() == U'=')
        {
            _sr->forward();
            return flushToken(tcTokenType::Equality);
        }
        return flushToken(tcTokenType::Equal);
    case U'≠':
        return flushToken(tcTokenType::NotEqual);
    default:
        return flushToken(tcTokenType::Bad);
    }
}

tcToken tcLexer::parseIdentifier()
{
    auto peek = _sr->peek();

    while (isLetter(peek) || isDigit(peek))
    {
        _sr->forward();
        peek = _sr->peek();
    }

    auto sourceSpan = _sr->flush();
    const auto str = sourceSpan.string();

    const static std::unordered_map<std::string_view, tcTokenType> keywords
    {
        {"NAND", tcTokenType::Nand},
        {"NOR", tcTokenType::Nor},
        {"XNOR", tcTokenType::Xnor},
        {"AND", tcTokenType::And},
        {"OR", tcTokenType::Or},
        {"XOR", tcTokenType::Xor},
        {"NOT", tcTokenType::Not},
        {"π", tcTokenType::Pi},
        {"τ", tcTokenType::Tau}
    };
    const auto typeIter = keywords.find(str);
    if (typeIter != keywords.end())
        return {typeIter->second, std::move(sourceSpan)};
    else
        return {tcTokenType::Identifier, std::move(sourceSpan)};
}

char32_t tcLexer::decimalSeparator() const
{
    return _commaArgumentSeparator ? U'.' : U',';
}

char32_t tcLexer::argSeparator() const
{
    return _commaArgumentSeparator ? U',' : U';';
}