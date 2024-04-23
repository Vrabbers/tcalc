#pragma once

#include <string>
#include "source_position.h"

namespace tc
{
    class source_span final
    {
    public:
        source_span(std::string&& str, const source_position start, const size_t src_ix) : _string(str),
            _position(start), _source_index(src_ix)
        {
        }

        [[nodiscard]]
        std::string_view source_str() const
        {
            return _string;
        }

        [[nodiscard]]
        source_position position() const
        {
            return _position;
        }

        [[nodiscard]]
        size_t source_index() const
        {
            return _source_index;
        }

    private:
        std::string _string;
        source_position _position;
        size_t _source_index;
    };
}
