#pragma once

#include <string_view>
#include <optional>
#include <string>
#include "source_position.h"

namespace TCalc
{
    constexpr char32_t EndOfFile = -1;

    class StringReader final
    {
        private:
            std::string string;
            std::size_t startIx;
            std::size_t endIx;
            SourcePosition startPosition;
            SourcePosition endPosition;

        public:
            StringReader(std::string input);
            std::pair<char32_t, std::size_t> peekNextCharAndLength() const;
            std::optional<char32_t> peekNextCharacter() const;
            std::optional<char32_t> moveNextCharacter();
            std::size_t tokenLength() const;
            std::string flushToken();
            void discardToken();
    };
};
