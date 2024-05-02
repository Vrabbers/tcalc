#include <array>
#include <iostream>
#include <format>
#include <random>
#include <cstring>

#include "expression.h"
#include "lexer.h"
#include "parser.h"
#include "operation.h"
#include "number.h"

#ifdef _WIN32
#include <windows.h>
#pragma execution_character_set("utf-8")
#endif

static tcalc::parser parse(std::string&& str)
{
    tcalc::lexer lexer{std::move(str), true};
    tcalc::parser parser{std::move(lexer)};

    return parser;
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
        auto p = parse(std::move(input));
        auto arith = p.parse_expression();
        for (const auto& op : std::get<tcalc::arithmetic_expression>(arith).tokens)
        {
            std::cout << tcalc::op_to_string(op) << ' ';
        }

        if (!p.diagnostic_bag().empty())
        {
            std::cout << "\n\x1b[31mdiagnostics:\n";
            for (const auto& diag : p.diagnostic_bag())
            {
                std::cout << std::format("{} @ {}-{} \n", tcalc::diagnostic_type_name(diag.type()), diag.start_index(), diag.end_index());
            }
            std::cout << "\x1b[0m";
        }
        std::cout << '\n';
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
        parse(std::move(a));
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

    mpfr_mp_memory_cleanup();
}