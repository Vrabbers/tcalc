#pragma once

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "tc_source_position.h"

namespace tcalc
{
    constexpr char32_t end_of_file = static_cast<char32_t>(-1);

    class string_reader final
    {
    public:
        explicit string_reader(std::string&& input) : _string{std::move(input)}
        {
        }

        explicit string_reader(const std::string& input) : _string{input}
        {
        }

        [[nodiscard]]
        std::optional<char32_t> peek() const;

        [[nodiscard]]
        std::u32string peek_many(int32_t count) const;

        [[nodiscard]]
        std::optional<char32_t> current() const
        {
            return _current;
        }

        std::optional<char32_t> forward();

        void forward_many(const int32_t count)
        {
            for (int32_t i = 0; i < count; i++)
                forward();
        }

        [[nodiscard]]
        size_t token_length() const
        {
            return _end_ix - _start_ix;
        }

        [[nodiscard]]
        std::pair<tcalc::source_position, std::string_view> flush();

        void discard_token()
        {
            _start_ix = _end_ix;
        }

    private:
        [[nodiscard]]
        std::pair<char32_t, size_t> peek_with_length() const;
        std::optional<char32_t> _current;
        std::string _string;
        size_t _start_ix = 0;
        size_t _end_ix = 0;
    };
}
