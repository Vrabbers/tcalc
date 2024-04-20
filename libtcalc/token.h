#pragma once

#include <memory>

#include "source_span.h"
#include "tcalc_export.h"
#include "token_type.h"

class tcToken final
{
    public:
        TCALC_EXPORT tcToken();
        TCALC_EXPORT tcToken(tcTokenType type, tcSourceSpan&& source);
        [[nodiscard]]
        TCALC_EXPORT tcTokenType type() const;
        [[nodiscard]]
        TCALC_EXPORT const tcSourceSpan& source() const;
        TCALC_EXPORT ~tcToken();

    private:
        tcTokenType _type;
        std::unique_ptr<tcSourceSpan> _source;
};
