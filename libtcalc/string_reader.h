#pragma once

#include <string_view>
#include <optional>
#include <string>
#include <memory>

#include "source_position.h"
#include "source_span.h"
#include "tcalc_export.h"

constexpr char32_t EndOfFile = (char32_t) -1;

class StringReader final
{
    public:
        explicit StringReader(std::string&& input);
        std::optional<char32_t> peekNextCharacter() const;
        std::optional<char32_t> moveNextCharacter();
        std::size_t tokenLength() const;
        SourceSpan flush();
        void discardToken();
    
    private:
        std::pair<char32_t, std::size_t> peekNextCharAndLength() const;
        std::string _string;
        std::size_t _startIx;
        std::size_t _endIx;
        SourcePosition _startPosition;
        SourcePosition _endPosition;
};
