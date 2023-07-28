#include "lexer.h"

#include "utf8proc.h"

bool isWhitespace(std::optional<char32_t> chr)
{
    if (!chr.has_value())
        return false;
    // tab is not in category ZS. conveniently, neither is LF.
    return chr == U'\t' || utf8proc_category(chr.value()) == UTF8PROC_CATEGORY_ZS;
}

bool isLetter(std::optional<char32_t> chr)
{
    if (!chr.has_value())
        return false;
    auto c = utf8proc_category(chr.value());
    return (c >= UTF8PROC_CATEGORY_LU && c <= UTF8PROC_CATEGORY_LO) || c == UTF8PROC_CATEGORY_PC;
}

bool isDigit(std::optional<char32_t> chr)
{
    return chr >= U'0' && chr <= U'9';
}

bool isHexDigit(std::optional<char32_t> chr)
{
    return isDigit(chr) || (chr >= U'a' && chr <= U'f') || (chr >= U'A' && chr <= U'F');
}

Lexer::Lexer(std::string&& input, bool argSeparatorIsComma)
    : _sr(std::move(input)), _argSeparatorIsComma(argSeparatorIsComma)
{
}

Token Lexer::next()
{
    auto chOpt = _sr.moveNextCharacter();

    if (!chOpt.has_value())
        return flushToken(Token::Type::Bad);
    
    auto character = chOpt.value();

    if (character == EndOfFile)
        return flushToken(Token::Type::EndOfFile);
    if (character == U'\n') //TODO: nicer check here
        return flushToken(Token::Type::EndOfLine);
    if (isDigit(character))
        return parseNumber(character);

    return flushToken(Token::Type::Bad);
}

Token Lexer::flushToken(Token::Type type)
{
    auto span = _sr.flush();
    return Token(type, span);
}

Token Lexer::parseNumber(char32_t first)
{
    auto next = _sr.peekNextCharacter();
    if (first == U'0' && next == U'b')
    {
        _sr.moveNextCharacter();
        while (_sr.peekNextCharacter() == U'0' || _sr.peekNextCharacter() == U'1')
            _sr.moveNextCharacter();

        return flushToken(Token::Type::BinaryLiteral);
    }

    if (first == U'0' && next == U'x')
    {
        _sr.moveNextCharacter();
        while (isHexDigit(_sr.peekNextCharacter()))
            _sr.moveNextCharacter();

        return flushToken(Token::Type::HexLiteral);
    }

    bool parsingAfterDecimal = false;
    bool parsingExponent = false;
    while (true)
    {
        next = _sr.peekNextCharacter();
        
        if (isDigit(next))
        {
            _sr.moveNextCharacter();
        }
        else if (next == decimalSeparator())
        {
            _sr.moveNextCharacter();

            if (!parsingAfterDecimal)
                parsingAfterDecimal = true;
            else
                return flushToken(Token::Type::Bad);
        }
        else if (next == U'e' || next == U'E')
        {
            _sr.moveNextCharacter();

            if (!parsingExponent)
            {
                parsingExponent = true;
                parsingAfterDecimal = true;
                next = _sr.peekNextCharacter();
                if (next == U'+' || next == U'-')
                    _sr.moveNextCharacter();
            }
            else
            {
                return flushToken(Token::Type::Bad);
            }
        }
        else
        {
            if (_sr.peekNextCharacter() == U'i')
                _sr.moveNextCharacter();
            
            return flushToken(Token::Type::NumericLiteral);
        }
    }
}

char32_t Lexer::decimalSeparator() const
{
    return _argSeparatorIsComma ? U'.' : U',';
}

char32_t Lexer::argSeparator() const
{
    return _argSeparatorIsComma ? U',' : U';';
}
