#include <cstddef>
#include <optional>
#include <set>
#include <string>
#include <functional>

#pragma once

namespace tcalc
{
    class string_reader final
    {
    private:
        std::string string;
        std::size_t start_ix; // inclusive
        std::size_t end_ix;   // exclusive
                              // i.e. given a string "abcde", start_ix = 0, end_ix = 1 gives "a", both 0 give ""
    public:
        // constructs based on string
        explicit string_reader(const std::string&& input);

        // peeks next character without advancing end index, returning std::nullopt on EOF
        [[nodiscard]]
        std::optional<std::string> peek_next_char() const;

        // gets next character and advances end. if at EOF, returns std::nullopt
        std::optional<std::string> move_next_char();

        // gets current length of token
        [[nodiscard]]
        std::size_t token_length() const;

        // flushes token, returning a copy of that substring and starts a new one at that end.
        std::string flush_token();

        // reads while predicate returns true
        void read_while(const std::function<bool(std::string&)>& predicate);
        // reads while character is one of characters
        void read_while(const std::set<std::string>& characters);
        // reads until predicate returns true (inverse of read_while)
        void read_until(const std::function<bool(std::string&)>& predicate);
        // reads until character is one of characters
        void read_until(const std::set<std::string>& characters);

        // alias for read_while with whitespace characters
        void skip_whitespace();
    };q
}