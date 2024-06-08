#include "tc_number.h"

#include <cassert>
#include <iostream>

#ifdef _MSC_VER
#pragma warning(push, 0) // mpc header has warnings on MSVC /W4
#endif

#include <mpc.h>

#ifdef _MSC_VER
#pragma warning(pop)
#endif


using namespace tcalc;

inline void assert_owns(const std::unique_ptr<number_pimpl>& p)
{
    assert(p != nullptr);
}

struct memory_stuff final
{
    ~memory_stuff()
    {
        mpfr_mp_memory_cleanup();
    }
};

struct tcalc::number_pimpl final
{
    mpc_t ref{};

    [[nodiscard]]
    mpfr_ptr real_ref()
    {
        return mpc_realref(ref);
    }

    [[nodiscard]]
    mpfr_srcptr real_ref() const
    {
        return mpc_realref(ref);
    }

    [[nodiscard]]
    mpfr_ptr imag_ref()
    {
        return mpc_imagref(ref);
    }

    [[nodiscard]]
    mpfr_srcptr imag_ref() const
    {
        return mpc_imagref(ref);
    }

    static memory_stuff memory_stuff_inst;
};

memory_stuff number_pimpl::memory_stuff_inst{};

static constexpr mpc_rnd_t round_mode = MPC_RNDNN;
static constexpr mpfr_rnd_t fr_round_mode = MPFR_RNDN;

static std::string make_mpfr_format(const std::string_view from)
{
    std::string str{};
    str.reserve(from.length()); // Most of the time, it will be just as long, and it won't ever be any longer

    for (const auto c : from)
    {
        if (c == '\'' || c == 'i') // Ignore imaginary i and thousands separator
            continue;
        if (c == ',' || c == '.') // Make either decimal point just a period, as MPFR will always accept it as decimal
            str.push_back('.');
        else
            str.push_back(c); // Otherwise just add the character on
    }

    return str;
}

number::number(const long precision) : d{std::make_unique<number_pimpl>()}
{
    mpc_init2(d->ref, precision);
    set(0, 0);
}

number::number(const number& other)
{
    *this = other;
}

number& number::operator=(const number& other)
{
    const auto prec = other.precision();
    d = std::make_unique<number_pimpl>();
    mpc_init2(d->ref, prec);
    mpc_set(d->ref, other.d->ref, round_mode);
    return *this;
}

number::number(number&& other) noexcept
{
    *this = std::move(other);
}

number& number::operator=(number&& other) noexcept
{
    d = std::move(other.d);
    return *this;
}

number::~number()
{
    if (d != nullptr)
        mpc_clear(d->ref);
}

void number::set(const long real, const long imaginary)
{
    assert_owns(d);
    mpc_set_si_si(d->ref, real, imaginary, round_mode);
}

void number::set(const number& other)
{
    assert_owns(d);
    mpc_set(d->ref, other.d->ref, round_mode);
}

void number::set_real(const std::string_view real)
{
    const auto string = make_mpfr_format(real);
    mpfr_set_str(d->real_ref(), string.c_str(), 10, fr_round_mode);
}

void number::set_imaginary(const std::string_view imaginary)
{
    const auto string = make_mpfr_format(imaginary);
    mpfr_set_str(d->imag_ref(), string.c_str(), 10, fr_round_mode);
}

void number::set_imaginary(const long im)
{
    mpfr_set_si(mpc_imagref(d->ref), im, fr_round_mode);
}

void number::set_binary(const std::string_view bin)
{
    const auto string = make_mpfr_format(bin);
    assert(string.length() >= 2);
    mpfr_set_str(d->real_ref(),  &string.c_str()[2], 2, fr_round_mode); // Cut 0b part off
}

void number::set_hexadecimal(const std::string_view hex)
{
    const auto string = make_mpfr_format(hex);
    assert(string.length() >= 2);
    mpfr_set_str(d->real_ref(), &string.c_str()[2], 16, fr_round_mode); // Cut 0x part off
}

bool number::is_real() const
{
    assert_owns(d);
    return mpfr_zero_p(d->imag_ref());
}

bool number::is_infinity() const
{
    return mpfr_inf_p(d->real_ref()) || mpfr_inf_p(d->imag_ref());
}

bool number::is_nan() const
{
    return mpfr_nan_p(d->real_ref()) || mpfr_nan_p(d->imag_ref());
}

bool number::is_integer() const
{
    if (!is_real())
        return false;
    return mpfr_integer_p(d->real_ref());
}

static std::string real_only_string(const mpc_t handle)
{
    const auto size = mpfr_snprintf(nullptr, 0, "%.18Rg", mpc_realref(handle));
    std::string str(static_cast<size_t>(size), '\0');
    mpfr_snprintf(str.data(), size + 1, "%.18Rg", mpc_realref(handle));
    return str;
}

static std::string both_string_positive_imaginary(const mpc_t handle)
{
    const auto size = mpfr_snprintf(nullptr, 0, "%.18Rg+%.18Rgi", mpc_realref(handle), mpc_imagref(handle));
    std::string str(static_cast<size_t>(size), '\0');
    mpfr_snprintf(str.data(), size + 1, "%.18Rg+%.18Rgi", mpc_realref(handle), mpc_imagref(handle));
    return str;
}

static std::string both_string_negative_imaginary(const mpc_t handle)
{
    const auto size = mpfr_snprintf(nullptr, 0, "%.18Rg%.18Rgi", mpc_realref(handle), mpc_imagref(handle));
    std::string str(static_cast<size_t>(size), '\0');
    mpfr_snprintf(str.data(), size + 1, "%.18Rg%.18Rgi", mpc_realref(handle), mpc_imagref(handle));
    return str;
}

static std::string both_string(const mpc_t handle)
{
    if (mpfr_sgn(mpc_imagref(handle)) < 0)
        return both_string_negative_imaginary(handle);
    return both_string_positive_imaginary(handle);
}

bool number::operator==(const long r) const
{
    assert_owns(d);
    return mpc_cmp_si_si(d->ref, r, 0) == 0;
}

bool number::operator<(const number& b) const
{
    assert_owns(d);
    const int res = mpc_cmp(d->ref, b.d->ref);
    return MPC_INEX_RE(res) < 0;
}

bool number::operator>(const number& b) const
{
    assert_owns(d);
    int res = mpc_cmp(d->ref, b.d->ref);
    return MPC_INEX_RE(res) > 0;
}

bool number::operator==(const number& b) const
{
    assert_owns(d);
    return mpc_cmp(d->ref, b.d->ref) == 0;
}

long number::precision() const
{
    return mpc_get_prec(d->ref);
}

void number::add(const number& lhs, const number& rhs)
{
    assert_owns(d);
    mpc_add(d->ref, lhs.d->ref, rhs.d->ref, round_mode);
}

void number::sub(const number& lhs, const number& rhs)
{
    assert_owns(d);
    mpc_sub(d->ref, lhs.d->ref, rhs.d->ref, round_mode);
}

void number::negate(const number& x)
{
    assert_owns(d);
    if (*this != 0)
        mpc_mul_si(d->ref, x.d->ref, -1, round_mode);
}

void number::mul(const number& lhs, const number& rhs)
{
    assert_owns(d);
    mpc_mul(d->ref, lhs.d->ref, rhs.d->ref, round_mode);
}

void number::mul(const number& lhs, const long rhs)
{
    assert_owns(d);
    mpc_mul_si(d->ref, lhs.d->ref, rhs, round_mode);
}

void number::div(const number& lhs, const number& rhs)
{
    assert_owns(d);
    mpc_div(d->ref, lhs.d->ref, rhs.d->ref, round_mode);
}

void number::div(const number &lhs, unsigned long rhs)
{
    mpc_div_ui(d->ref, lhs.d->ref, rhs, round_mode);
}

void number::pow(const number& lhs, const number& rhs)
{
    assert_owns(d);
    mpc_pow(d->ref, lhs.d->ref, rhs.d->ref, round_mode);
}

void number::sqrt(const number& x)
{
    assert_owns(d);
    mpc_sqrt(d->ref, x.d->ref, round_mode);
}

void number::reciprocal(const number& x)
{
    assert_owns(d);
    mpc_pow_si(d->ref, x.d->ref, -1, round_mode);
}

void number::reciprocal(const long x)
{
    assert_owns(d);
    set(x);
    reciprocal(*this);
}

void number::nth_root(const number& x, const number& root)
{
    assert_owns(d);
    number recip{precision()};
    recip.reciprocal(root);
    mpc_pow(d->ref, x.d->ref, recip.d->ref, round_mode);
}

void number::nth_root(const number& x, const long root)
{
    if (root % 2 == 1)
    {
        set(x);
    }
    assert_owns(d);
    number recip{precision()};
    recip.reciprocal(root);
    mpc_pow(d->ref, d->ref, recip.d->ref, round_mode);
}

void number::exp(const number& x)
{
    assert_owns(d);
    mpc_exp(d->ref, x.d->ref, round_mode);
}

void number::log(const number& x)
{
    assert_owns(d);
    mpc_log10(d->ref, x.d->ref, round_mode);
}

void number::ln(const number& x)
{
    assert_owns(d);
    mpc_log(d->ref, x.d->ref, round_mode);
}

void number::sin(const number& x)
{
    assert_owns(d);
    const number pi = number::pi(x.precision());
    number k{x.precision()};
    k.div(x, pi);
    if (k.is_integer())
        set(0);
    else
        mpc_sin(d->ref, x.d->ref, round_mode);
}

void number::cos(const number& x)
{
    assert_owns(d);
    const number pi = number::pi(x.precision());
    number k{x.precision()};
    k.div(x, pi);
    mpfr_sub_d(k.d->real_ref(), k.d->real_ref(), 0.5, fr_round_mode);
    if (k.is_integer())
        set(0);
    else
        mpc_cos(d->ref, x.d->ref, round_mode);
}

void number::tan(const number& x)
{
    assert_owns(d);
    const number pi = number::pi(x.precision());
    number k{x.precision()};
    k.div(x, pi);
    if (k.is_integer())
        set(0);
    else
        mpc_tan(d->ref, x.d->ref, round_mode);
}

void number::abs(const number& x)
{
    assert_owns(d);
    mpc_abs(d->real_ref(), x.d->ref, fr_round_mode);
    set_imaginary(0);
}

std::string number::string() const
{
    if (is_real())
        return real_only_string(d->ref);
    return both_string(d->ref);
}

std::string number::dbg_string() const
{
    char* a = mpc_get_str(10, 0, d->ref, fr_round_mode);
    std::string str{a};
    mpc_free_str(a);
    return str;
}

number number::pi(const long prec)
{
    number pi{prec};
    mpfr_const_pi(pi.d->real_ref(), fr_round_mode);
    return pi;
}

number number::tau(const long prec)
{
    number tau = pi(prec);
    mpc_mul_si(tau.d->ref, tau.d->ref, 2, round_mode);
    return tau;
}

number number::e(const long prec)
{
    number e{prec};
    mpfr_t fr_one;
    mpfr_init_set_si(fr_one, 1, fr_round_mode);
    mpfr_exp(e.d->real_ref(), fr_one, fr_round_mode);
    mpfr_clear(fr_one);
    return e;
}
