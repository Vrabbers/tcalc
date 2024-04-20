#include <iostream>
#include <format>
#include "lexer.h"

#ifdef _WIN32
#include <windows.h>
#pragma execution_character_set("utf-8")
#endif

int main()
{
#ifdef _WIN32
    SetConsoleCP(65001);
    SetConsoleOutputCP(65001);
#endif
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
        std::cout << "input:\n" << input;

        tcLexer lexer(input.c_str(), true);

        while (true)
        {
            tcToken next = lexer.next();
            auto l = next.source().position().line;
            auto c = next.source().position().column;
            std::cout << std::format("{} \"{}\" (L:{} C:{})\n", tcTokenTypeName(next.type()), next.source().string(), l, c);
            if (next.type() == tcTokenType::EndOfFile)
                break;
        }
    }
}