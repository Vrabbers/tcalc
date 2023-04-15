#include <iostream>
#include "string_reader.h"

std::ostream& operator<<(std::ostream& out, const std::optional<std::string>& opt_char) 
{
    if (opt_char.has_value())
        out << "make_optional(\"" << std::hex << opt_char.value() << "\") ";
    else 
        out << "nullopt ";
    return out;
}

tcalc::string_reader i_return_a_reader()
{
    std::string hello;
    std::getline(std::cin, hello);
    tcalc::string_reader reader(std::move(hello));
    return reader;
}

int main()
{
    while (true)
    {
        std::cout << "turtle calculator\n";

        auto reader = i_return_a_reader();

        auto res1 = reader.move_next_char();
        auto res2 = reader.move_next_char();
        auto res3 = reader.move_next_char();

        auto token_length = reader.token_length();
        auto flush = reader.flush_token();

        std::cout << res1 << res2 << res3 << '\n';
        std::cout << token_length << flush << "\n";

        reader.skip_whitespace();

        auto res4 = reader.move_next_char();
        std::cout << res4;
    }
    return 0;
}

