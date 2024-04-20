#pragma once

#include <optional>
#include <string>

#include "source_position.h"
#include "source_span.h"

constexpr char32_t EndOfFile = static_cast<char32_t>(-1);

class tcStringReader final
{
public:
    explicit tcStringReader(std::string&& input);
    explicit tcStringReader(const char* input);
    [[nodiscard]]
    std::optional<char32_t> peek() const;
    std::optional<char32_t> forward();
    [[nodiscard]]
    std::size_t tokenLength() const;
    [[nodiscard]]
    tcSourceSpan flush();
    void discardToken();

private:
    [[nodiscard]]
    std::pair<char32_t, std::int32_t> peekAndLength() const;
    std::string _string;
    std::size_t _startIx = 0;
    std::size_t _endIx = 0;
    tcSourcePosition _startPosition = {1, 1};
    tcSourcePosition _endPosition = {1, 1};
};
