#pragma once

#include <string_view>
#include <optional>
#include <string>
#include <memory>

#include "source_position.h"
#include "tcalc_export.h"

namespace TCalc
{
    constexpr char32_t EndOfFile = (char32_t) - 1;

    // TODO: only exported for debugging purposes. make header private and un-export after this is done
    class TCALC_EXPORT StringReader final
    {
        private:
            std::unique_ptr<std::string> string;
            std::size_t startIx;
            std::size_t endIx;
            SourcePosition startPosition;
            SourcePosition endPosition;

        public:
            explicit StringReader(std::unique_ptr<std::string>);
            std::pair<char32_t, std::size_t> peekNextCharAndLength() const;
            std::optional<char32_t> peekNextCharacter() const;
            std::optional<char32_t> moveNextCharacter();
            std::size_t tokenLength() const;
            std::string flushToken();
            void discardToken();
    };
};
