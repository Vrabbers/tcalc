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

        void set_binary(std::string_view bin);

        void set_hexadecimal(std::string_view hex);

        [[nodiscard]]
        bool is_real() const;

        [[nodiscard]]
        bool operator==(long r) const;

        [[nodiscard]]
        bool operator<(const number& b) const;

        [[nodiscard]]
        bool operator>(const number& b) const;

        [[nodiscard]]
        bool operator==(const number& b) const;

        void add(const number& lhs, const number& rhs);

        void sub(const number& lhs, const number& rhs);

        void negate(const number& x);

        void mul(const number& lhs, const number& rhs);

        void div(const number& lhs, const number& rhs);

        void div(const number& lhs, unsigned long rhs);

        void pow(const number& lhs, const number& rhs);

        void sqrt(const number& x);

        void exp(const number& x);

        void log(const number& x);

        void ln(const number& x);

        void sin(const number& x);

        void cos(const number& x);

        void tan(const number& x);

        [[nodiscard]]
        std::string string() const;

        static number pi(long prec);
        static number tau(long prec);
        static number e(long prec);

    private:
        [[nodiscard]]
        bool owns() const;

        std::unique_ptr<number_pimpl> d;
    };

    void teardown();
}

#endif // TC_NUMBER_H
