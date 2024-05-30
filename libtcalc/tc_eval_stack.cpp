#include "tc_eval_stack.h"

tcalc::number& tcalc::eval_stack::pop()
{
    // We keep old entries to avoid creating and destroying perfectly good number instances.
    auto& num = _stack.at(_top);
    _top--;
    return num;
}

void tcalc::eval_stack::push(const number& num)
{
    if (static_cast<int>(_stack.size()) == _top + 1)
    {
        number num2{num};
        _stack.push_back(std::move(num2));
    }
    else
    {
        _stack.at(_top + 1).set(num);
    }
    _top++;
}
