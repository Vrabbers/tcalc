#include "tc_number.h"

#include <iostream>

static std::string make_mpfr_format(std::string_view from)
{
    std::string str{};
    str.reserve(from.length()); // Most of the time, it will be just as long, and it won't ever be any longer

    for (auto c : from)
    {
        if (c == '\'' || c == 'i') // Ignore imaginary i and thousands separator
            continue;
        if (c == ',' || c == '.') // Make either decimal point just a period, as MPFR will always accept it as dcimal
            str.push_back('.');
        else
            str.push_back(c); // Otherwise just add the character on
    }
    return str;
}

void tcalc::number::set_real(std::string_view real)
{
    auto string = make_mpfr_format(real);
    mpfr_set_str(mpc_realref(_handle), string.c_str(), 10, fr_round_mode);
}

void tcalc::number::set_imaginary(std::string_view imaginary)
{
    auto string = make_mpfr_format(imaginary);
    mpfr_set_str(mpc_imagref(_handle), string.c_str(), 10, fr_round_mode);
}

static std::string real_only_string(const mpc_t handle)
{
    auto size = mpfr_snprintf(nullptr, 0, "%.18Rg", mpc_realref(handle)) + 1;
    std::string str(static_cast<size_t>(size), '\0');
    mpfr_snprintf(str.data(), size, "%.18Rg", mpc_realref(handle));
    return str;
}

static std::string both_string_positive_imaginary(const mpc_t handle)
{
    auto size = mpfr_snprintf(nullptr, 0, "%.18Rg+%.18Rgi", mpc_realref(handle), mpc_imagref(handle)) + 1;
    std::string str(static_cast<size_t>(size), '\0');
    mpfr_snprintf(str.data(), size, "%.18Rg+%.18Rgi", mpc_realref(handle), mpc_imagref(handle));
    return str;
}

static std::string both_string_negative_imaginary(const mpc_t handle)
{
    auto size = mpfr_snprintf(nullptr, 0, "%.18Rg%.18Rgi", mpc_realref(handle), mpc_imagref(handle)) + 1;
    std::string str(static_cast<size_t>(size), '\0');
    mpfr_snprintf(str.data(), size, "%.18Rg%.18Rgi", mpc_realref(handle), mpc_imagref(handle));
    return str;
}

static std::string both_string(const mpc_t handle)
{
    if (mpfr_sgn(mpc_imagref(handle)) < 0)
        return both_string_negative_imaginary(handle);
    return both_string_positive_imaginary(handle);
}

std::string tcalc::number::string() const
{
    if (is_real())
        return real_only_string(_handle);
    return both_string(_handle);
}
