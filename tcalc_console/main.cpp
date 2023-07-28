#include <iostream>
#include "lexer.h"
#include "tc_utf8_utils.h"

void printToken(SourceSpan);

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

    Lexer lexer(std::move(input), true);
    auto next = lexer.next();
    std::cout << Token::typeName(next.type());
}