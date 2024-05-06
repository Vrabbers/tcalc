#pragma once

#include <memory>
#include <string_view>
#include <utility>
#include <vector>

#include "tc_source_position.h"

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
        diagnostic(const source_position pos, const diagnostic_type type) :
            _position{pos},
            _type{type}
        {
        }

        diagnostic(const source_position pos, const diagnostic_type type, std::vector<std::string>&& arguments) :
            _position{pos},
            _arguments{std::move(arguments)},
            _type{type}
        {
        }

        diagnostic(const source_position pos, const diagnostic_type type, const std::string_view argument) :
            _position{pos},
            _arguments{std::string{argument}},
            _type{type}
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
            return _position.start_index;
        }

        [[nodiscard]]
        size_t end_index() const
        {
            return _position.end_index;
        }

        [[nodiscard]]
        const source_position& position() const
        {
            return _position;
        }

        [[nodiscard]]
        const std::vector<std::string>& arguments() const
        {
            return _arguments;
        }

    private:
        source_position _position;
        std::vector<std::string> _arguments;
        diagnostic_type _type;
    };
}
