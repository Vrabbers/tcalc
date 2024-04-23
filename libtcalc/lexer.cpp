#include "lexer.h"

#include <set>
#include <unordered_map>

#include "utf8_utils.h"

static bool is_whitespace(std::optional<char32_t> chr)
{
    if (!chr.has_value())
        return false;
    // tab is not in category ZS. conveniently, neither is LF.
    return chr == U'\t' || utf8proc::category(*chr) == UTF8PROC_CATEGORY_ZS;
}

static bool is_letter(std::optional<char32_t> chr)
{
    if (!chr.has_value())
        return false;
    auto c = utf8proc::category(*chr);
    return (c >= UTF8PROC_CATEGORY_LU && c <= UTF8PROC_CATEGORY_LO)
        || c == UTF8PROC_CATEGORY_PC
        || c == UTF8PROC_CATEGORY_NO;
}

static bool is_decimal_digit(std::optional<char32_t> chr)
{
    return chr >= U'0' && chr <= U'9';
}

static bool is_hex_digit(std::optional<char32_t> chr)
{
    return is_decimal_digit(chr) || (chr >= U'a' && chr <= U'f') || (chr >= U'A' && chr <= U'F');
}

static const std::set superscript_digits = {U'²', U'³', U'¹', U'⁰', U'⁴', U'⁵', U'⁶', U'⁷', U'⁸', U'⁹'};

static bool is_superscript_digit(std::optional<char32_t> chr)
{
    if (!chr.has_value())
        return false;
    return superscript_digits.contains(*chr);
}

tc::token tc::lexer::next()
{
    while (is_whitespace(_sr->peek()))
        _sr->forward();

    _sr->discard_token();

    auto first = _sr->forward();

    if (!first.has_value())
    {
        auto token = flush_token(token_kind::bad);
        _diagnostic_bag->insert(diagnostic(token.source(), diagnostic_type::bad_character));
        return token;
    }

    if (*first == END_OF_FILE)
        return flush_token(token_kind::end_of_file);
    if (*first == U'\n')
        return flush_token(token_kind::end_of_line);
    if (is_decimal_digit(first))
        return lex_number();
    if (is_superscript_digit(first))
        return lex_superscript_number();
    if (is_letter(first))
        return lex_word();

    return lex_symbol();
}

tc::token tc::lexer::flush_token(token_kind type)
{
    auto span = _sr->flush();
    return {type, std::move(span)};
}

tc::token tc::lexer::lex_number()
{
    auto first = _sr->current();
    auto next = _sr->peek();
    if (first == U'0' && next == U'b')
    {
        _sr->forward();
        while (_sr->peek() == U'0' || _sr->peek() == U'1' || _sr->peek() == U'_')
            _sr->forward();

        return flush_token(token_kind::binary_literal);
    }

    if (first == U'0' && next == U'x')
    {
        _sr->forward();
        while (is_hex_digit(_sr->peek()) || _sr->peek() == U'_')
            _sr->forward();

        return flush_token(token_kind::hex_literal);
    }

    bool reading_decimal = false;
    bool reading_exponent = false;
    while (true)
    {
        next = _sr->peek();

        if (is_decimal_digit(next) || next == U'_')
        {
            _sr->forward();
        }
        else if (next == decimal_separator())
        {
            _sr->forward();

            if (!reading_decimal && !reading_exponent)
            {
                reading_decimal = true;
            }
            else
            {
                auto token = flush_token(token_kind::bad);
                _diagnostic_bag->insert(diagnostic(token.source(), diagnostic_type::bad_number_literal));
                return token;
            }
        }
        else if (next == U'e' || next == U'E')
        {
            if (!reading_exponent)
            {
                auto next3 = _sr->peek_many(3);
                if (next3.length() >= 2) // Otherwise we don't have enough input to keep going
                {
                    if (next3[1] == U'+' || next3[1] == U'-')
                    {
                        if (next3.length() == 3 && is_decimal_digit(next3[2])) // If it was a +/-, we need to consume a digit
                        {
                            reading_exponent = true;
                            _sr->forward_many(3);
                            continue;
                        }
                    }
                    else if (is_decimal_digit(next3[1])) // If is already a digit, try to read more
                    {
                        reading_exponent = true;
                        _sr->forward_many(2);
                        continue;
                    }
                }
            }
            // In any other case, finish reading early.
            return flush_token(token_kind::numeric_literal);
        }
        else
        {
            if (_sr->peek() == U'i')
                _sr->forward();

            return flush_token(token_kind::numeric_literal);
        }
    }
}

tc::token tc::lexer::lex_superscript_number()
{
    bool reading_exponent = false;
    while (true)
    {
        auto next = _sr->peek();

        if (is_superscript_digit(next))
        {
            _sr->forward();
        }
        else if (next == U'ᵉ' || next == U'ᴱ')
        {
            // cf. similar code in lex_number()
            if (!reading_exponent)
            {
                auto next3 = _sr->peek_many(3);
                if (next3.length() >= 2)
                {
                    if (next3[1] == U'⁺' || next3[1] == U'⁻')
                    {
                        if (next3.length() == 3 && is_superscript_digit(next3[2]))
                        {
                            reading_exponent = true;
                            _sr->forward_many(3);
                            continue;
                        }
                    }
                    else if (is_superscript_digit(next3[1]))
                    {
                        reading_exponent = true;
                        _sr->forward_many(2);
                        continue;
                    }
                }
            }
            return flush_token(token_kind::superscript_literal);
        }
        else
        {
            if (_sr->peek() == U'ⁱ')
                _sr->forward();

            return flush_token(token_kind::superscript_literal);
        }
    }
}

tc::token tc::lexer::lex_symbol()
{
    if (_sr->current() == arg_separator())
        return flush_token(token_kind::argument_separator);

    std::optional<char32_t> next;
    switch (*_sr->current())
    {
        case U'+':
            return flush_token(token_kind::plus);
        case U'⁺':
            return flush_token(token_kind::superscript_plus);
        case U'-':
            return flush_token(token_kind::minus);
        case U'⁻':
            return flush_token(token_kind::superscript_minus);
        case U'*':
        case U'×':
        case U'∙':
            return flush_token(token_kind::multiply);
        case U'/':
        case U'÷':
            return flush_token(token_kind::divide);
        case U'^':
            return flush_token(token_kind::exponentiate);
        case U'(':
            return flush_token(token_kind::left_parenthesis);
        case U')':
            return flush_token(token_kind::right_parenthesis);
        case U'√':
            return flush_token(token_kind::radical);
        case U'%':
            return flush_token(token_kind::percent);
        case U'!':
            if (_sr->peek() == U'=')
            {
                _sr->forward();
                return flush_token(token_kind::not_equal);
            }
            return flush_token(token_kind::factorial);
        case U'>':
            next = _sr->peek();
            if (next == U'>')
            {
                _sr->forward();
                return flush_token(token_kind::right_shift);
            }
            if (next == U'=')
            {
                _sr->forward();
                return flush_token(token_kind::greater_or_equal);
            }
            return flush_token(token_kind::greater);
        case U'<':
            next = _sr->peek();
            if (next == U'<')
            {
                _sr->forward();
                return flush_token(token_kind::left_shift);
            }
            if (next == U'=')
            {
                _sr->forward();
                return flush_token(token_kind::less_or_equal);
            }
            return flush_token(token_kind::less);
        case U'≥':
            return flush_token(token_kind::greater_or_equal);
        case U'≤':
            return flush_token(token_kind::less_or_equal);
        case U'=':
            if (_sr->peek() == U'=')
            {
                _sr->forward();
                return flush_token(token_kind::equality);
            }
            return flush_token(token_kind::equal);
        case U'≠':
            return flush_token(token_kind::not_equal);
        default:
            auto token = flush_token(token_kind::bad);
            _diagnostic_bag->insert(diagnostic(token.source(), diagnostic_type::bad_symbol));
            return token;
    }
}

tc::token tc::lexer::lex_word()
{
    auto peek = _sr->peek();

    while (is_letter(peek) || is_decimal_digit(peek))
    {
        _sr->forward();
        peek = _sr->peek();
    }

    auto sourceSpan = _sr->flush();
    const auto str = sourceSpan->source_str();

    const static std::unordered_map<std::string_view, token_kind> keywords
    {
        {"NAND", token_kind::binary_nand},
        {"NOR", token_kind::binary_nor},
        {"XNOR", token_kind::binary_xnor},
        {"AND", token_kind::binary_and},
        {"OR", token_kind::binary_or},
        {"XOR", token_kind::binary_xor},
        {"NOT", token_kind::binary_not},
        {"π", token_kind::pi},
        {"τ", token_kind::tau},
        {"i", token_kind::numeric_literal},
        {"ⁱ", token_kind::superscript_literal},
    };

    const auto type_iter = keywords.find(str);
    if (type_iter != keywords.end())
        return {type_iter->second, std::move(sourceSpan)};

    return {token_kind::identifier, std::move(sourceSpan)};
}

char32_t tc::lexer::decimal_separator() const
{
    return _comma_argument_separator ? U'.' : U',';
}

char32_t tc::lexer::arg_separator() const
{
    return _comma_argument_separator ? U',' : U';';
}
