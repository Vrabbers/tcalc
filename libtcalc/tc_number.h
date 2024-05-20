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
            if (_owns)
                mpc_set_si_si(_handle, real, imaginary, round_mode);
        }

        void set(const number& other)
        {
            assert(_owns);
            if (_owns)
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
            if (_owns)
                return mpfr_zero_p(mpc_imagref(_handle));
            return false;
        }

        void add(const number& a, const number& b)
        {
            assert(_owns);
            if (_owns)
                mpc_add(_handle, a._handle, b._handle, round_mode);
        }

        void sqrt(const number& a)
        {
            assert(_owns);
            if (_owns)
                mpc_sqrt(_handle, a._handle, round_mode);
        }

        void exp(const number& a)
        {
            assert(_owns);
            if (_owns)
                mpc_exp(_handle, a._handle, round_mode);
        }

        void log(const number& a)
        {
            assert(_owns);
            if (_owns)
                mpc_log10(_handle, a._handle, round_mode);
        }

        void ln(const number& a)
        {
            assert(_owns);
            if (_owns)
                mpc_log(_handle, a._handle, round_mode);
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
