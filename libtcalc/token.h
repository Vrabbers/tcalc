#pragma once

#include "source_span.h"
#include "tcalc_export.h"
#include "token_type.h"

class tcToken final
{
    public:
        TCALC_EXPORT explicit tcToken(tcTokenType type, tcSourceSpan source);
        TCALC_EXPORT tcTokenType type() const;
        TCALC_EXPORT tcSourceSpan source() const;
        TCALC_EXPORT ~tcToken() = default;
    private:
        tcTokenType _type;
        tcSourceSpan _source;
};
