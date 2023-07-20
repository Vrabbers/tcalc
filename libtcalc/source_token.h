#pragma once

#include <string>
#include "source_position.h"

namespace TCalc
{
    class SourceToken final
    {
        public:
            SourceToken(std::string, SourcePosition start, SourcePosition end);
            const std::string& getString() const;
            SourcePosition getStart() const;
            SourcePosition getEnd() const;
        private:
            std::string string;
            SourcePosition start;
            SourcePosition end;
    };
}