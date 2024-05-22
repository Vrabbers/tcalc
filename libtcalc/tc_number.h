#ifndef TC_NUMBER_H
#define TC_NUMBER_H

#ifdef _MSC_VER
#pragma warning(push, 0) // mpc header has warnings on MSVC /Wall
#endif

#include <mpc.h>

#ifdef _MSC_VER
#pragma warning(pop)
#endif

#include <cassert>
#include <string>

namespace tcalc
{
    class number final
    {
    public:
        static constexpr mpc_rnd_t round_mode = MPC_RNDNN;
        static constexpr mpfr_rnd_t fr_round_mode = MPFR_RNDN;

        explicit number(const mpfr_prec_t precision)
        {
            mpc_init2(_handle, precision);
            set(0, 0);
        }

        number(const number& other)
        {
            const auto prec = mpc_get_prec(other._handle);
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

        void set(const long real, const long imaginary = 0)
        {
            assert(_owns);
            mpc_set_si_si(_handle, real, imaginary, round_mode);
        }

        void set(const number& other)
        {
            assert(_owns);
            mpc_set(_handle, other._handle, round_mode);
        }

        void set_real(std::string_view real);

        void set_imaginary(std::string_view imaginary);

        void set_binary(std::string_view bin);

        void set_hexadecimal(std::string_view hex);

        [[nodiscard]]
        bool is_real() const
        {
            assert(_owns);
            return mpfr_zero_p(mpc_imagref(_handle));
        }

        [[nodiscard]]
        bool is_zero() const
        {
            assert(_owns);
            return mpc_cmp_si(_handle, 0) == 0;
        }

        void add(const number& rhs)
        {
            assert(_owns);
            mpc_add(_handle, _handle, rhs._handle, round_mode);
        }

        void sub(const number& rhs)
        {
            assert(_owns);
            mpc_sub(_handle, _handle, rhs._handle, round_mode);
        }

        void negate()
        {
            assert(_owns);
            mpc_mul_si(_handle, _handle, -1, round_mode);
        }

        void mul(const number& rhs)
        {
            assert(_owns);
            mpc_mul(_handle, _handle, rhs._handle, round_mode);
        }

        void div(const number& rhs)
        {
            assert(_owns);
            mpc_div(_handle, _handle, rhs._handle, round_mode);
        }

        void pow(const number& rhs)
        {
            assert(_owns);
            mpc_pow(_handle, _handle, rhs._handle, round_mode);
        }

        void sqrt()
        {
            assert(_owns);
            mpc_sqrt(_handle, _handle, round_mode);
        }

        void exp()
        {
            assert(_owns);
            mpc_exp(_handle, _handle, round_mode);
        }

        void log()
        {
            assert(_owns);
            mpc_log10(_handle, _handle, round_mode);
        }

        void ln()
        {
            assert(_owns);
            mpc_log(_handle, _handle, round_mode);
        }

        [[nodiscard]]
        std::string string() const;

        static number pi(mpfr_prec_t prec);
        static number tau(mpfr_prec_t prec);
        static number e(mpfr_prec_t prec);
    private:
        mpc_t _handle{};
        bool _owns{true};
    };
}

#endif // TC_NUMBER_H
