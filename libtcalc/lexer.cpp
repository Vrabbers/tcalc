#include "lexer.h"

#include <set>
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

static const std::set superscriptNumerals = {U'²', U'³', U'¹', U'⁰', U'⁴', U'⁵', U'⁶', U'⁷', U'⁸', U'⁹'};

static bool isSuperscriptNumeral(std::optional<char32_t> chr)
{
    if (!chr.has_value())
        return false;
    return superscriptNumerals.contains(*chr);
}

tcToken tcLexer::next()
{
    while (isWhitespace(_sr->peek()))
    {
        _sr->forward();
    }
    _sr->discardToken();

    auto first = _sr->forward();

    if (!first.has_value())
        return flushToken(tcTokenKind::Bad);
    if (*first == EndOfFile)
        return flushToken(tcTokenKind::EndOfFile);
    if (*first == U'\n')
        return flushToken(tcTokenKind::EndOfLine);
    if (isDigit(first))
        return parseNumber(*first);
    if (isSuperscriptNumeral(first))
        return parseSuperscriptNumber();
    if (isLetter(first))
        return parseIdentifier();

    return parseSymbol(*first);
}

tcToken tcLexer::flushToken(tcTokenKind type)
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

        return flushToken(tcTokenKind::BinaryLiteral);
    }

    if (first == U'0' && next == U'x')
    {
        _sr->forward();
        while (isHexDigit(_sr->peek()) || _sr->peek() == U'_')
            _sr->forward();

        return flushToken(tcTokenKind::HexLiteral);
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
                return flushToken(tcTokenKind::Bad);
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
                return flushToken(tcTokenKind::Bad);
            }
        }
        else
        {
            if (_sr->peek() == U'i')
                _sr->forward();

            return flushToken(tcTokenKind::NumericLiteral);
        }
    }
}

tcToken tcLexer::parseSuperscriptNumber()
{
    bool parsingExponent = false;
    while (true)
    {
        auto next = _sr->peek();

        if (isSuperscriptNumeral(next))
        {
            _sr->forward();
        }
        else if (next == U'ᵉ' || next == U'ᴱ')
        {
            _sr->forward();

            if (!parsingExponent)
            {
                parsingExponent = true;
                next = _sr->peek();
                if (next == U'⁺' || next == U'⁻')
                    _sr->forward();
            }
            else
            {
                return flushToken(tcTokenKind::Bad);
            }
        }
        else
        {
            if (_sr->peek() == U'ⁱ')
                _sr->forward();

            return flushToken(tcTokenKind::SuperscriptLiteral);
        }
    }
}

tcToken tcLexer::parseSymbol(char32_t first)
{
    if (first == argSeparator())
        return flushToken(tcTokenKind::ArgumentSeparator);

    std::optional<char32_t> next;
    switch (first)
    {
    case U'+':
        return flushToken(tcTokenKind::Plus);
    case U'⁺':
        return flushToken(tcTokenKind::SuperscriptPlus);
    case U'-':
        return flushToken(tcTokenKind::Minus);
    case U'⁻':
        return flushToken(tcTokenKind::SuperscriptMinus);
    case U'*':
    case U'×':
    case U'∙':
        return flushToken(tcTokenKind::Multiply);
    case U'/':
    case U'÷':
        return flushToken(tcTokenKind::Divide);
    case U'^':
        return flushToken(tcTokenKind::Exponentiate);
    case U'(':
        return flushToken(tcTokenKind::LeftParens);
    case U')':
        return flushToken(tcTokenKind::RightParens);
    case U'√':
        return flushToken(tcTokenKind::Radical);
    case U'%':
        return flushToken(tcTokenKind::Percent);
    case U'!':
        if (_sr->peek() == U'=')
        {
            _sr->forward();
            return flushToken(tcTokenKind::NotEqual);
        }
        return flushToken(tcTokenKind::Factorial);
    case U'>':
        next = _sr->peek();
        if (next == U'>')
        {
            _sr->forward();
            return flushToken(tcTokenKind::RightShift);
        }
        if (next == U'=')
        {
            _sr->forward();
            return flushToken(tcTokenKind::GreaterOrEqual);
        }
        return flushToken(tcTokenKind::Greater);
    case U'<':
        next = _sr->peek();
        if (next == U'<')
        {
            _sr->forward();
            return flushToken(tcTokenKind::LeftShift);
        }
        if (next == U'=')
        {
            _sr->forward();
            return flushToken(tcTokenKind::LessOrEqual);
        }
        return flushToken(tcTokenKind::Less);
    case U'≥':
        return flushToken(tcTokenKind::GreaterOrEqual);
    case U'≤':
        return flushToken(tcTokenKind::LessOrEqual);
    case U'=':
        if (_sr->peek() == U'=')
        {
            _sr->forward();
            return flushToken(tcTokenKind::Equality);
        }
        return flushToken(tcTokenKind::Equal);
    case U'≠':
        return flushToken(tcTokenKind::NotEqual);
    default:
        return flushToken(tcTokenKind::Bad);
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
    const auto str = sourceSpan.sourceStr();

    const static std::unordered_map<std::string_view, tcTokenKind> keywords
    {
        {"NAND", tcTokenKind::Nand},
        {"NOR", tcTokenKind::Nor},
        {"XNOR", tcTokenKind::Xnor},
        {"AND", tcTokenKind::And},
        {"OR", tcTokenKind::Or},
        {"XOR", tcTokenKind::Xor},
        {"NOT", tcTokenKind::Not},
        {"π", tcTokenKind::Pi},
        {"τ", tcTokenKind::Tau},
        {"i", tcTokenKind::NumericLiteral},
        {"ⁱ", tcTokenKind::SuperscriptLiteral},
    };

    const auto typeIter = keywords.find(str);
    if (typeIter != keywords.end())
        return {typeIter->second, std::move(sourceSpan)};

    return {tcTokenKind::Identifier, std::move(sourceSpan)};
}

char32_t tcLexer::decimalSeparator() const
{
    return _commaArgumentSeparator ? U'.' : U',';
}

char32_t tcLexer::argSeparator() const
{
    return _commaArgumentSeparator ? U',' : U';';
}