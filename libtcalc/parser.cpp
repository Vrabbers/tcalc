#include "parser.h"

using namespace tcalc;

static bool ends_expr(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::expression_separator:
        case token_kind::end_of_file:
            return true;
        default:
            return false;
    }
}

expression parser::parse_expression()
{
    return {};
}
