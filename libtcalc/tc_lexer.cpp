#include "tc_lexer.h"

#include <unordered_map>
#include <optional>

#include "utf8utils.h"

using namespace tcalc;

namespace
{
    bool is_whitespace(const std::optional<char32_t> chr)
    {
        if (!chr.has_value())
            return false;
        // tab is not in category ZS. conveniently, neither is LF.
        return chr == U'\t' || utf8utils::category(*chr) == UTF8PROC_CATEGORY_ZS;
    }

    bool is_letter(const std::optional<char32_t> chr)
    {
        if (!chr.has_value())
            return false;
        const auto c = utf8utils::category(*chr);
        return (c >= UTF8PROC_CATEGORY_LU && c <= UTF8PROC_CATEGORY_LO)
            || c == UTF8PROC_CATEGORY_PC
            || c == UTF8PROC_CATEGORY_NO;
    }

    bool is_decimal_digit(const std::optional<char32_t> chr)
    {
        return chr >= U'0' && chr <= U'9';
    }

    bool is_hex_digit(const std::optional<char32_t> chr)
    {
        return is_decimal_digit(chr) || (chr >= U'a' && chr <= U'f') || (chr >= U'A' && chr <= U'F');
    }

    bool is_superscript_digit(const std::optional<char32_t> chr)
    {
        if (!chr.has_value())
            return false;
        switch (*chr)
        {
            case U'²':
            case U'³':
            case U'¹':
            case U'⁰':
            case U'⁴':
            case U'⁵':
            case U'⁶':
            case U'⁷':
            case U'⁸':
            case U'⁹':
                return true;
            default:
                return false;
        }
    }

    consteval char32_t plus(bool is_superscript)
    {
        return is_superscript ? U'⁺' : U'+';
    }

    consteval char32_t minus(bool is_superscript)
    {
        return is_superscript ? U'⁻' : U'-';
    }

    template<bool Superscript>
    bool is_digit(char32_t chr)
    {
        if constexpr (Superscript)
            return is_superscript_digit(chr);
        else
            return is_decimal_digit(chr);
    }

    template<bool Superscript>
    bool start_reading_exponent(string_reader& sr)
    {
        const auto next3 = sr.peek_many(3);
        if (next3.length() >= 2) // Otherwise we don't have enough input to keep going
        {
            if (next3[1] == plus(Superscript) || next3[1] == minus(Superscript))
            {
                if (next3.length() == 3 && is_digit<Superscript>(next3[2])) // If it was a +/-, we need to consume a digit
                {
                    sr.forward_many(3);
                    return true;
                }
            }
            else if (is_digit<Superscript>(next3[1]))// If is already a digit, need to read them and then start reading exponent
            {
                sr.forward_many(2);
                return true;
            }
        }
        return false; // If not enough input, or if characters we didnt want were there
    }
} // End anonymous namespace

token lexer::next()
{
    while (is_whitespace(_sr.peek()))
        _sr.forward();

    _sr.discard_token();

    const auto first = _sr.forward();

    if (!first.has_value())
    {
        auto token = flush_token(token_kind::bad);
        _diagnostic_bag.emplace_back(token.position(), diagnostic_type::bad_character);
        return token;
    }

    if (*first == end_of_file)
    {
        _reached_end = true;
        return flush_token(token_kind::end_of_file);
    }
    if (is_decimal_digit(first))
        return lex_number();
    if (is_superscript_digit(first))
        return lex_superscript_number();
    if (is_letter(first))
        return lex_word();

    return lex_symbol();
}

token lexer::flush_token(const token_kind kind)
{
    const auto [pos, str_view] = _sr.flush();
    return {kind, std::string{str_view}, pos};
}

token lexer::lex_binary_number()
{
    _sr.forward();
    while (_sr.peek() == U'0' || _sr.peek() == U'1' || _sr.peek() == U'_')
        _sr.forward();

    return flush_token(token_kind::binary_literal);
}

token lexer::lex_hex_number()
{
    _sr.forward();
    while (is_hex_digit(_sr.peek()) || _sr.peek() == U'_')
        _sr.forward();

    return flush_token(token_kind::hex_literal);
}

token lexer::lex_decimal_number()
{
    bool reading_decimal = false;
    bool reading_exponent = false;
    while (true)
    {
        const auto next = _sr.peek();

        if (is_decimal_digit(next) || next == U'\'')
        {
            _sr.forward();
        }
        else if (next == decimal_separator())
        {
            _sr.forward();

            if (!reading_decimal && !reading_exponent)
            {
                reading_decimal = true;
            }
            else
            {
                auto token = flush_token(token_kind::bad);
                _diagnostic_bag.emplace_back(token.position(), diagnostic_type::invalid_number_literal);
                return token;
            }
        }
        else if (next == U'e' || next == U'E')
        {
            if (!reading_exponent)
            {
                reading_exponent = start_reading_exponent<false>(_sr);
                if (reading_exponent)
                    continue;
            }
            // If we didn't start reading exponent, finish early.
            return flush_token(token_kind::numeric_literal);
        }
        else
        {
            if (next == U'i')
                _sr.forward();

            return flush_token(token_kind::numeric_literal);
        }
    }
}

token lexer::lex_number()
{
    const auto first = _sr.current();
    const auto next = _sr.peek();
    if (first == U'0' && next == U'b')
        return lex_binary_number();

    if (first == U'0' && next == U'x')
        return lex_hex_number();

    return lex_decimal_number();
}

token lexer::lex_superscript_number()
{
    bool reading_exponent = false;
    while (true)
    {
        const auto next = _sr.peek();

        if (is_superscript_digit(next))
        {
            _sr.forward();
        }
        else if (next == U'ᵉ' || next == U'ᴱ')
        {
            if (!reading_exponent)
            {
                reading_exponent = start_reading_exponent<true>(_sr);
                if (reading_exponent)
                    continue;
            }

            return flush_token(token_kind::superscript_literal);
        }
        else
        {
            if (_sr.peek() == U'ⁱ')
                _sr.forward();

            return flush_token(token_kind::superscript_literal);
        }
    }
}

token lexer::lex_symbol()
{
    if (_sr.current() == arg_separator())
        return flush_token(token_kind::argument_separator);

    std::optional<char32_t> next;
    switch (*_sr.current())
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
        case U'⋅':
            return flush_token(token_kind::multiply);
        case U'/':
        case U'÷':
            return flush_token(token_kind::divide);
        case U'^':
            return flush_token(token_kind::exponentiate);
        case U'(':
            return flush_token(token_kind::open_parenthesis);
        case U')':
            return flush_token(token_kind::close_parenthesis);
        case U'√':
            return flush_token(token_kind::radical);
        case U'%':
            return flush_token(token_kind::percent);
        case U'!':
            if (_sr.peek() == U'=')
            {
                _sr.forward();
                return flush_token(token_kind::not_equal);
            }
            return flush_token(token_kind::factorial);
        case U'>':
            next = _sr.peek();
            if (next == U'>')
            {
                _sr.forward();
                return flush_token(token_kind::right_shift);
            }
            if (next == U'=')
            {
                _sr.forward();
                return flush_token(token_kind::greater_or_equal);
            }
            return flush_token(token_kind::greater_than);
        case U'<':
            next = _sr.peek();
            if (next == U'<')
            {
                _sr.forward();
                return flush_token(token_kind::left_shift);
            }
            if (next == U'=')
            {
                _sr.forward();
                return flush_token(token_kind::less_or_equal);
            }
            return flush_token(token_kind::less_than);
        case U'≥':
            return flush_token(token_kind::greater_or_equal);
        case U'≤':
            return flush_token(token_kind::less_or_equal);
        case U'=':
            if (_sr.peek() == U'=')
            {
                _sr.forward();
                return flush_token(token_kind::equality);
            }
            return flush_token(token_kind::equal);
        case U'≠':
            return flush_token(token_kind::not_equal);
        case U'\n':
        case U':':
            return flush_token(token_kind::expression_separator);
        default:
            auto token = flush_token(token_kind::bad);
            _diagnostic_bag.emplace_back(token.position(), diagnostic_type::invalid_symbol);
            return token;
    }
}

token lexer::lex_word()
{
    auto peek = _sr.peek();

    while ((is_letter(peek) || is_decimal_digit(peek)) && !is_superscript_digit(peek))
    {
        _sr.forward();
        peek = _sr.peek();
    }

    const auto [pos, str_view] = _sr.flush();

    const static std::unordered_map<std::string_view, token_kind> keywords
    {
        {"NAND", token_kind::binary_nand},
        {"NOR", token_kind::binary_nor},
        {"XNOR", token_kind::binary_xnor},
        {"AND", token_kind::binary_and},
        {"OR", token_kind::binary_or},
        {"XOR", token_kind::binary_xor},
        {"NOT", token_kind::binary_not},
        {"i", token_kind::numeric_literal},
        {"ⁱ", token_kind::superscript_literal},
    };

    const auto type_iter = keywords.find(str_view);
    if (type_iter != keywords.end())
        return {type_iter->second, std::string{str_view}, pos};

    return {token_kind::identifier, std::string{str_view}, pos};
}
