#include "tc_number.h"

#include <cassert>
#include <iostream>

using namespace tcalc;

static std::string make_mpfr_format(const std::string_view from)
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

void number::set_real(const std::string_view real)
{
    const auto string = make_mpfr_format(real);
    mpfr_set_str(mpc_realref(_handle), string.c_str(), 10, fr_round_mode);
}

void number::set_imaginary(const std::string_view imaginary)
{
    const auto string = make_mpfr_format(imaginary);
    mpfr_set_str(mpc_imagref(_handle), string.c_str(), 10, fr_round_mode);
}

void number::set_binary(const std::string_view bin)
{
    const auto string = make_mpfr_format(bin);
    assert(string.length() >= 2);
    mpfr_set_str(mpc_realref(_handle),  &string.c_str()[2], 2, fr_round_mode); // Cut 0b part off
}

void number::set_hexadecimal(const std::string_view hex)
{
    const auto string = make_mpfr_format(hex);
    assert(string.length() >= 2);
    mpfr_set_str(mpc_realref(_handle), &string.c_str()[2], 16, fr_round_mode); // Cut 0x part off
}

static std::string real_only_string(const mpc_t handle)
{
    const auto size = mpfr_snprintf(nullptr, 0, "%.18Rg", mpc_realref(handle)) + 1;
    std::string str(static_cast<size_t>(size), '\0');
    mpfr_snprintf(str.data(), size, "%.18Rg", mpc_realref(handle));
    return str;
}

static std::string both_string_positive_imaginary(const mpc_t handle)
{
    const auto size = mpfr_snprintf(nullptr, 0, "%.18Rg+%.18Rgi", mpc_realref(handle), mpc_imagref(handle)) + 1;
    std::string str(static_cast<size_t>(size), '\0');
    mpfr_snprintf(str.data(), size, "%.18Rg+%.18Rgi", mpc_realref(handle), mpc_imagref(handle));
    return str;
}

static std::string both_string_negative_imaginary(const mpc_t handle)
{
    const auto size = mpfr_snprintf(nullptr, 0, "%.18Rg%.18Rgi", mpc_realref(handle), mpc_imagref(handle)) + 1;
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

std::string number::string() const
{
    if (is_real())
        return real_only_string(_handle);
    return both_string(_handle);
}
