#pragma once
#include <vector>

#include "tc_number.h"

namespace tcalc
{
    class eval_stack final
    {
    public:
        [[nodiscard]]
        number& top()
        {
            return _stack.at(_top);
        }

        [[nodiscard]]
        bool has_at_least(const int amt) const
        {
            return amt <= size();
        }

        [[nodiscard]]
        int size() const
        {
            return _top + 1;
        }

        number& pop();

        void push(const number& num);

    private:
        std::vector<number> _stack;
        int _top = -1;
    };
}
