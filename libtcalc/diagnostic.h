#pragma once

#include <memory>
#include <utility>
#include <vector>

#include "source_span.h"

namespace tcalc
{
    enum class diagnostic_type
    {
        bad_character,
        invalid_number_literal,
        invalid_symbol,
        unexpected_token
    };

    std::string_view diagnostic_type_name(diagnostic_type type);

    class diagnostic final
    {
    public:
        diagnostic(const source_span& src, const diagnostic_type type) :
            _start_index{src.start_index()}, _end_index{src.end_index()}, _type{type}
        {
        }

        diagnostic(const source_span& src, const diagnostic_type type, std::vector<std::string>&& arguments) :
            _start_index{src.start_index()}, _end_index{src.end_index()}, _arguments{std::move(arguments)}, _type{type}
        {
        }

        [[nodiscard]]
        diagnostic_type type() const
        {
            return _type;
        }

        [[nodiscard]]
        size_t start_index() const
        {
            return _end_index;
        }

        [[nodiscard]]
        size_t end_index() const
        {
            return _end_index;
        }

    private:
        size_t _start_index;
        size_t _end_index;
        std::vector<std::string> _arguments;
        diagnostic_type _type;
    };
}
