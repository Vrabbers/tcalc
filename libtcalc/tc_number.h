#ifndef TC_NUMBER_H
#define TC_NUMBER_H

#include <memory>
#include <string>

namespace tcalc
{
    struct number_pimpl;

    class number final
    {
    public:
        explicit number(long precision);

        number(const number& other);

        number& operator=(const number& other);

        number(number&& other) noexcept;

        number& operator=(number&& other) noexcept;

        ~number();

        void set(long real, long imaginary = 0);

        void set(const number& other);

        void set_real(std::string_view real);

        void set_imaginary(std::string_view imaginary);

        void set_imaginary(long im);

        void set_binary(std::string_view bin);

        void set_hexadecimal(std::string_view hex);

        [[nodiscard]]
        bool is_real() const;

        [[nodiscard]]
        bool is_infinity() const;

        [[nodiscard]]
        bool is_nan() const;

        [[nodiscard]]
        bool is_integer() const;

        [[nodiscard]]
        bool operator==(long r) const;

        [[nodiscard]]
        bool operator<(const number& b) const;

        [[nodiscard]]
        bool operator>(const number& b) const;

        [[nodiscard]]
        bool operator==(const number& b) const;

        [[nodiscard]]
        long precision() const;

        void add(const number& lhs, const number& rhs);

        void sub(const number& lhs, const number& rhs);

        void negate(const number& x);

        void mul(const number& lhs, const number& rhs);

        void mul(const number& lhs, long rhs);

        void div(const number& lhs, const number& rhs);

        void div(const number& lhs, unsigned long rhs);

        void pow(const number& lhs, const number& rhs);

        void sqrt(const number& x);

        void reciprocal(const number& x);

        void reciprocal(long x);

        void nth_root(const number& x, const number& root);

        void nth_root(const number& x, long root);

        void exp(const number& x);

        void log(const number& x);

        void ln(const number& x);

        void sin(const number& x);

        void cos(const number& x);

        void tan(const number& x);

        void abs(const number& x);

        void re(const number& x);

        void im(const number& x);

        void arg(const number& x);

        void conj(const number& x);

        void asin(const number& x);

        void acos(const number& x);

        void atan(const number& x);

        void sinh(const number& x);
        void cosh(const number& x);
        void tanh(const number& x);
        void asinh(const number& x);
        void acosh(const number& x);
        void atanh(const number& x);

        [[nodiscard]]
        std::string string() const;

        [[nodiscard]]
        std::string dbg_string() const;

        static number pi(long prec);
        static number tau(long prec);
        static number e(long prec);

    private:
        std::unique_ptr<number_pimpl> d;
    };
}

#endif // TC_NUMBER_H
