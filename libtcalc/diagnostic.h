#pragma once

#include <memory>
#include <utility>

#include "source_span.h"

namespace tcalc
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
        size_t start_index() const
        {
            return _source_span.start_index();
        }

        [[nodiscard]]
        size_t end_index() const
        {
            return _source_span.end_index();
        }

    private:
        source_span _source_span;
        diagnostic_type _type;
    };
}
