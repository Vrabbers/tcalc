#include <iostream>
#include "string_reader.h"

void printToken(TCalc::SourceToken);

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

    TCalc::StringReader sr(input);
    sr.moveNextCharacter();
    auto token = sr.flushToken();
    printToken(token);
}

void printToken(TCalc::SourceToken token)
{
    std::cout << "source token information:\n" << token.getString() << std::endl;
    std::cout << "start -- line: " << token.getStart().line << "  column: " << token.getStart().column;
    std::cout << "\nend -- line: " << token.getEnd().line << "  column: " << token.getEnd().column;

}