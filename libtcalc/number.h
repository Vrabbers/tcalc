#pragma once

#ifdef _MSC_VER
#pragma warning(push, 0) // mpc header has warnings on MSVC /Wall
#endif

#include <mpc.h>

#ifdef _MSC_VER
#pragma warning(pop)
#endif

#include <optional>

namespace tcalc
{
    class number final
    {
    public:
        static constexpr mpc_rnd_t round_mode = MPC_RNDNN;

        explicit number(mpfr_prec_t precision)
        {
            mpc_init2(_handle, precision);
        }

        number(const number& other)
        {
            auto prec = mpc_get_prec(other._handle);
            mpc_init2(_handle, prec);
            mpc_set(_handle, other._handle, round_mode);
        }

        number(number&& other) noexcept
        {
            mpc_swap(_handle, other._handle);
            other._owns = false;
        }

        ~number()
        {
            if (_owns)
                mpc_clear(_handle);
        }

        void square()
        {
            if (_owns)
                mpc_pow_si(_handle, _handle, 2, round_mode);
        }

        void set(const long real, const long imaginary = 0)
        {
            if (_owns)
                mpc_set_si_si(_handle, real, imaginary, round_mode);
        }

        void add(const number& a, const number& b)
        {
            if (_owns)
                mpc_add(_handle, a._handle, b._handle, round_mode);
        }

        [[nodiscard]]
        std::string string() const
        {
            if (!_owns)
                return {};

            char* str = mpc_get_str(10, 0, _handle, round_mode);
            auto string = std::string{str};
            mpc_free_str(str);
            return string;
        }

    private:
        mpc_t _handle{};
        bool _owns{true};
    };
}
