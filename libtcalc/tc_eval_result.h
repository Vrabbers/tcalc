#ifndef TC_EVAL_RESULT_H
#define TC_EVAL_RESULT_H
#include <string_view>
#include <variant>

#include "tc_source_position.h"

namespace tcalc
{
    enum class eval_error_type
    {
        none = 0,
        invalid_program,
        divide_by_zero,
        log_zero,
        undefined_variable,
        undefined_function,
        log_base,
        bad_arity,
        complex_inequality,
    };

    std::string_view eval_error_type_name(eval_error_type);

    struct eval_error final
    {
        eval_error_type type;
        source_position position;
    };

    struct empty_result {};

    template<class SuccessType>
    class eval_result final
    {
    public:
        static eval_result from_error(const eval_error_type error, const source_position where)
        {
            eval_result res{};
            res._value = eval_error{error, where};
            return res;
        }

        static eval_result from_error(eval_error error)
        {
            eval_result res{};
            res._value = error;
            return res;
        }

        explicit eval_result(SuccessType&& success)
        {
            _value.template emplace<SuccessType>(std::move(success));
        }

        eval_result() = default;

        [[nodiscard]]
        bool is_error() const
        {
            return std::holds_alternative<eval_error>(_value);
        }

        [[nodiscard]]
        const eval_error& error() const
        {
            return std::get<eval_error>(_value);
        }

        [[nodiscard]]
        const SuccessType& value() const
        {
            return std::get<SuccessType>(_value);
        }

        SuccessType& mut_value()
        {
            return std::get<SuccessType>(_value);
        }
    private:
        std::variant<eval_error, SuccessType> _value;
    };
}

#endif
