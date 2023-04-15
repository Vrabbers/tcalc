#include "string_reader.h"
#include <vector>
#include <bit>

namespace tcalc
{
    const std::set<std::string> whitespace = {" ", "\t", "\n"};

    string_reader::string_reader(const std::string&& input) 
    {
        string = input;
        start_ix = 0;
        end_ix = 0;
    }

    std::optional<std::string> string_reader::peek_next_char() const
    {
        if (end_ix >= string.length())
        {
            return std::nullopt;
        }
        else
        {
            auto fst_chr = (unsigned char) string[end_ix];

            if (fst_chr < 0b10000000)
                return std::string(1, (char) fst_chr);

            auto length = std::countl_one(fst_chr);
            
            if (length == 1 || length > 4)
                throw "invalid utf8 string"; // this doesn't catch *everything*, but should catch most
            
            return string.substr(end_ix, length);
        }
    }

    std::optional<std::string> string_reader::move_next_char()
    {
        auto next = peek_next_char();
        if (next.has_value())
            end_ix += next.value().length();
        return next;
    }

    std::size_t string_reader::token_length() const
    {
        return end_ix - start_ix;
    }

    std::string string_reader::flush_token()
    {
        auto token = string.substr(start_ix, token_length());
        start_ix = end_ix;
        return token;
    }

    void string_reader::read_while(const std::function<bool(std::string&)>& predicate)
    {
        while (true)
        {
            auto next = peek_next_char();
            if (next.has_value() && predicate(next.value()))
            {
                end_ix++;
                continue;
            }
            // else
            break;
        }
    }

    void string_reader::read_while(const std::set<std::string>& characters)
    {
        read_while([&characters](std::string& c)
                   { return characters.contains(c); });
    }

    void string_reader::skip_whitespace()
    {
        read_while(whitespace);
        start_ix = end_ix;
    }
}