#pragma once

#include <string>

namespace tcalc
{
    class source_span final
    {
    public:
        source_span() : _start_index{}
        {
        }

        source_span(std::string&& str, const size_t src_ix) :
            _string{std::move(str)}, _start_index{src_ix}
        {
        }

        [[nodiscard]]
        std::string_view source_str() const
        {
            return _string;
        }

        [[nodiscard]]
        size_t start_index() const
        {
            return _start_index;
        }

        [[nodiscard]]
        size_t end_index() const
        {
            return start_index() + _string.length();
        }

    private:
        std::string _string;
        size_t _start_index;
    };
}
