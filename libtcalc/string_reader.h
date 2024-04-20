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
        std::optional<char32_t> peekNextCharacter() const;
        std::optional<char32_t> moveNextCharacter();
        [[nodiscard]]
        std::size_t tokenLength() const;
        [[nodiscard]]
        tcSourceSpan flush();
        void discardToken();

    private:
        std::pair<char32_t, std::size_t> peekNextCharAndLength() const;
        std::string _string;
        std::size_t _startIx;
        std::size_t _endIx;
        tcSourcePosition _startPosition;
        tcSourcePosition _endPosition;
};
