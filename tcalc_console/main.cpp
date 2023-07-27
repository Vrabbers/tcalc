#include <iostream>
#include "string_reader.h"
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

    StringReader sr(input);
    auto chr = sr.moveNextCharacter();
    std::cout << tcEncodeCodepoint(chr.value());
    sr.moveNextCharacter();
    auto token = sr.flushToken();
    printToken(token);
}

void printToken(SourceSpan token)
{
    std::cout << "source token information:\n" << token.string() << std::endl;
    std::cout << "start -- line: " << token.position().line << "  column: " << token.position().column;
}