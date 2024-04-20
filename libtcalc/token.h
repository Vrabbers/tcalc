#pragma once

#include <memory>

#include "source_span.h"
#include "token_type.h"

class tcToken final
{
    public:
        tcToken();
        tcToken(tcTokenType type, tcSourceSpan&& source);
        [[nodiscard]]
        tcTokenType type() const;
        [[nodiscard]]
        const tcSourceSpan& source() const;
        ~tcToken();

    private:
        tcTokenType _type;
        std::unique_ptr<tcSourceSpan> _source;
};
