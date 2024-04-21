#pragma once

#include <memory>

#include "source_span.h"
#include "token_kind.h"

class tcToken final
{
public:
    tcToken() : _kind(tcTokenKind::Bad)
    {
    }

    tcToken(const tcTokenKind kind, tcSourceSpan&& source) : _kind(kind),
                                                             _sourceSpan(std::make_unique<tcSourceSpan>(source))
    {
    }

    [[nodiscard]]
    tcTokenKind kind() const
    {
        return _kind;
    }

    [[nodiscard]]
    const tcSourceSpan& source() const
    {
        return *_sourceSpan;
    }

    [[nodiscard]]
    std::string_view sourceStr() const
    {
        return _sourceSpan->sourceStr();
    }

    [[nodiscard]]
    tcSourcePosition position() const
    {
        return _sourceSpan->position();
    }

    [[nodiscard]]
    std::string format() const;

private:
    tcTokenKind _kind;
    std::unique_ptr<tcSourceSpan> _sourceSpan;
};
