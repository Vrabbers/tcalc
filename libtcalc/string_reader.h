#pragma once

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "source_position.h"
#include "source_span.h"

constexpr char32_t EndOfFile = static_cast<char32_t>(-1);

class tcStringReader final
{
public:
    explicit tcStringReader(std::string&& input) : _string(std::move(input))
    {
    }

    explicit tcStringReader(const char* input) : _string(input)
    {
    }

    [[nodiscard]]
    std::optional<char32_t> peek() const;

    [[nodiscard]]
    std::u32string peekMany(std::uint32_t count) const;

    [[nodiscard]]
    std::optional<char32_t> current() const
    {
        return _current;
    }

    std::optional<char32_t> forward();

    void forwardMany(const std::uint32_t count)
    {
        for (std::uint32_t i = 0; i < count; i++)
            forward();
    }

    [[nodiscard]]
    std::size_t tokenLength() const
    {
        return _endIx - _startIx;
    }

    [[nodiscard]]
    tcSourceSpan flush();

    void discardToken();

private:
    [[nodiscard]]
    std::pair<char32_t, std::ptrdiff_t> peekAndLength() const;
    std::optional<char32_t> _current;
    std::string _string;
    std::size_t _startIx = 0;
    std::size_t _endIx = 0;
    tcSourcePosition _startPosition = {1, 1};
    tcSourcePosition _endPosition = {1, 1};
};
