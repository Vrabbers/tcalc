#include <array>
#include <iostream>
#include <format>
#include <random>
#include <cstring>

#include "lexer.h"

#ifdef _WIN32
#include <windows.h>
#pragma execution_character_set("utf-8")
#endif

static tcalc::lexer parse(std::string&& str, bool print = true)
{
    tcalc::lexer lexer(std::move(str), true);

    while (true)
    {
        tcalc::token next = lexer.next();
        if (print)
            std::cout << next.format() << '\n';
        if (next.kind() == tcalc::token_kind::end_of_file)
            break;
    }

    return lexer;
}

static void interactive()
{
    std::cout << "tcalc console\n";

    while (true)
    {
        std::string input;
        std::cout << "> ";
        std::getline(std::cin, input);

        if (input == "quit")
            return;

        std::cout << "\ninput:\n" << input << '\n';
        auto lex = parse(std::move(input));
        for (const auto& diag : lex.diagnostic_bag())
        {
            std::cout << std::format("{} @ {}-{} \n", diagnostic_type_name(diag.type()), diag.start_index(), diag.end_index());
        }
        std::cout << "\x1b[0m\n";
    }
}

static void fuzz(const int times)
{
    std::random_device rd;
    std::default_random_engine rand(rd());
    std::uniform_int_distribution rdist(0, 255);
    std::array<char, 512> buf{};
    for (int i = 0; i < times; i++)
    {
        for (char& j : buf)
            j = static_cast<char>(rdist(rand));
        std::cout << "Fuzz #" << i + 1 << '\n';
        std::string a{buf.cbegin(), buf.cend()};
        parse(std::move(a), false);
    }
}

int main(int argc, char* argv[])
{
#ifdef _WIN32
    SetConsoleCP(CP_UTF8);
    SetConsoleOutputCP(CP_UTF8);
#endif
    if (argc == 3 && std::strcmp(argv[1], "fuzz") == 0)
        fuzz(std::stoi(argv[2]));
    else
        interactive();
}