#pragma once
#include "source_span.h"
#include <memory>
#include <set>
#include <utility>

namespace tc
{
    enum class diagnostic_type
    {
        bad_character,
        invalid_number_literal,
        invalid_symbol
    };

    std::string_view diagnostic_type_name(diagnostic_type type);

    class diagnostic final
    {
    public:
        // Constructor makes a copy of the source_span, so it can live longer than the token which originally owned it.
        diagnostic(source_span  src, diagnostic_type type) : _source_span{std::move(src)}, _type{type}
        {
        }

        [[nodiscard]]
        diagnostic_type type() const
        {
            return _type;
        }

        [[nodiscard]]
        source_position position() const
        {
            return _source_span.position();
        }

        [[nodiscard]]
        const source_span& source() const
        {
            return _source_span;
        }

        [[nodiscard]]
        std::string_view source_str() const
        {
            return _source_span.source_str();
        }

        [[nodiscard]]
        size_t source_index() const
        {
            return _source_span.source_index();
        }

    private:
        source_span _source_span;
        diagnostic_type _type;
    };
}
