#pragma once

#include <memory>

#include "source_span.h"
#include "token_kind.h"

namespace tc
{
    class token final
    {
    public:
        token() : _kind(token_kind::bad)
        {
        }

        token(const token_kind kind, std::unique_ptr<source_span>&& source) : _kind(kind), _source_span(std::move(source))
        {
        }

        [[nodiscard]]
        token_kind kind() const
        {
            return _kind;
        }

        [[nodiscard]]
        const source_span& source() const
        {
            return *_source_span;
        }

        [[nodiscard]]
        std::string_view source_str() const
        {
            return _source_span->source_str();
        }

        [[nodiscard]]
        source_position position() const
        {
            return _source_span->position();
        }

        [[nodiscard]]
        std::string format() const;

    private:
        token_kind _kind;
        std::unique_ptr<source_span> _source_span;
    };
}
