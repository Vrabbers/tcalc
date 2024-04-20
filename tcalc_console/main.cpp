#include <iostream>
#include <cstdio>
#include "lexer.h"
#include "utf8_utils.h"

int main()
{
    std::cout << "tcalc console" << std::endl;
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
        std::printf("%s \"%s\" (L:%o C:%o)\n", tcTokenTypeName(next.type()), next.source().string(), l, c);
        if (next.type() == tcTokenType::EndOfFile)
            break;
    }
}