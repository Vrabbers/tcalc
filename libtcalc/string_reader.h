#pragma once

#include <cstdint>
#include <memory>
#include <optional>
#include <string>

#include "source_position.h"
#include "source_span.h"

namespace tc
{
    constexpr char32_t END_OF_FILE = static_cast<char32_t>(-1);

    class string_reader final
    {
    public:
        explicit string_reader(std::string&& input) : _string(std::move(input))
        {
            }

        explicit string_reader(const char* input) : _string(input)
        {
        }

        [[nodiscard]]
        std::optional<char32_t> peek() const;

        [[nodiscard]]
        std::u32string peek_many(std::uint32_t count) const;

        [[nodiscard]]
        std::optional<char32_t> current() const
        {
            return _current;
        }

        std::optional<char32_t> forward();

        void forward_many(const std::uint32_t count)
        {
            for (std::uint32_t i = 0; i < count; i++)
                forward();
        }

        [[nodiscard]]
        std::size_t token_length() const
        {
            return _end_ix - _start_ix;
        }

        [[nodiscard]]
        std::unique_ptr<source_span> flush();

        void discard_token()
        {
            _start_ix = _end_ix;
            _start_position = _end_position;
        }

    private:
        [[nodiscard]]
        std::pair<char32_t, std::ptrdiff_t> peek_with_length() const;
        std::optional<char32_t> _current;
        std::string _string;
        std::size_t _start_ix = 0;
        std::size_t _end_ix = 0;
        source_position _start_position = {1, 1};
        source_position _end_position = {1, 1};
    };
}
