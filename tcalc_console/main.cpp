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

static tcLexer parse(std::string&& str, bool print = true)
{
    tcLexer lexer(std::move(str), true);

    while (true)
    {
        tcToken next = lexer.next();
        if (print)
            std::cout << next.format() << std::endl;
        if (next.kind() == tcTokenKind::EndOfFile)
            break;
    }

    return lexer;
}

static void interactive()
{
    std::cout << "tcalc console" << std::endl;

    while (true)
    {
        std::string input;
        std::string aux;
        while (true)
        {
            std::cout << "> ";
            std::getline(std::cin, aux);
            if (aux.empty())
                break;
            else
                input += aux + "\n";
        }
        if (input.empty())
            return;

        std::cout << "input:\n" << input << std::endl;
        auto lex = parse(std::move(input));
        std::cout << "Diagnostics:" << std::endl;
        for (const auto& diag : lex.diagnosticBag())
        {
            std::cout << std::format("{} @ L:{}, C:{}", tcDiagnosticTypeName(diag.type()), diag.position().line, diag.position().column) << std::endl;
        }
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
        std::cout << "Fuzz #" << i + 1 << std::endl;
        std::string a{buf.cbegin(), buf.cend()};
        auto lex = parse(std::move(a), false);
        std::cout << "Diagnostics:" << std::endl;
        for (const auto& diag : lex.diagnosticBag())
        {
            std::cout << std::format("{} @ L:{}, C:{}", tcDiagnosticTypeName(diag.type()), diag.position().line, diag.position().column) << std::endl;
        }
    }
}

int main(int argc, char* argv[])
{
#ifdef _WIN32
    SetConsoleCP(65001);
    SetConsoleOutputCP(65001);
#endif
    if (argc == 3 && std::strcmp(argv[1], "fuzz") == 0)
        fuzz(std::stoi(argv[2]));
    else
        interactive();
}