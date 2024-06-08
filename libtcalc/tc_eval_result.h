#ifndef TC_EVAL_RESULT_H
#define TC_EVAL_RESULT_H

#include <string_view>
#include <variant>

#include "tc_source_position.h"
#include "tc_number.h"

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
        out_of_tan_domain,
        zero_pow_zero,
        assign_to_constant,
        zero_root,
        real_mode_complex_result,
        overflow,
        nan_error,
    };

    std::string_view eval_error_type_name(eval_error_type);

    struct eval_error final
    {
        eval_error_type type;
        source_position position;
    };

    struct assign_result final
    {
        std::string variable;
        number value;
    };

    template<class SuccessType>
    class eval_result final
    {
    public:
        eval_result(const eval_error_type error, const source_position where)
        {
            _value = eval_error{error, where};
        }

        explicit eval_result(eval_error error)
        {
            _value = error;
        }

        explicit eval_result(const SuccessType& success)
        {
            _value = success;
        }

        explicit eval_result(SuccessType&& success)
        {
            _value = std::move(success);
        }

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
