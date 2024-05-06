#include <array>
#include <iostream>
#include <format>
#include <random>
#include <cstring>
#include <iomanip>

#include "tc_expression.h"
#include "tc_lexer.h"
#include "tc_operation.h"
#include "tc_number.h"
#include "tc_parser.h"

#ifdef _WIN32
#include <windows.h>
#pragma execution_character_set("utf-8")
#endif

static tcalc::parser parse(std::string&& str)
{
    tcalc::lexer lexer{std::move(str), true};
    tcalc::parser parser{std::move(lexer), 64};

    return parser;
}

static void show_arith(const tcalc::arithmetic_expression& arith)
{
    for (const auto& op : arith.tokens)
    {
        std::cout << op_to_string(op) << ' ';
    }
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

        std::cout << "input: " << input;
        std::cout << "\n\nparse:\n";
        auto p = parse(std::move(input));
        for (const auto& expr : p.parse_all())
        {
            if (const auto* arith = std::get_if<tcalc::arithmetic_expression>(&expr))
            {
                std::cout << "arithmetic ";
                show_arith(*arith);
                std::cout << " @" << arith->position.start_index << '-' << arith->position.end_index;
            }
            else if (const auto* asgn = std::get_if<tcalc::assignment_expression>(&expr))
            {
                std::cout << "assigning to " << asgn->variable << ": ";
                show_arith(asgn->expression);
                std::cout << " @" << asgn->position.start_index << '-' << asgn->position.end_index;
            }
            else if (const auto* blnexp = std::get_if<tcalc::boolean_expression>(&expr))
            {
                std::cout << "boolean " << std::quoted(tcalc::token_kind_name(blnexp->kind)) << " with ";
                show_arith(blnexp->lhs);
                std::cout << "and ";
                show_arith(blnexp->rhs);
                std::cout << " @" << blnexp->position.start_index << '-' << blnexp->position.end_index;
            }
            else if (const auto* fndef = std::get_if<tcalc::func_def_expression>(&expr))
            {
                std::cout << "defining function " << fndef->name << " as ";
                show_arith(fndef->expression);
                std::cout << " @" << fndef->position.start_index << '-' << fndef->position.end_index;
            }
            std::cout << '\n';
        }
        if (!p.diagnostic_bag().empty())
        {
            std::cout << "\n\x1b[31mdiagnostics:\n";
            for (const auto& diag : p.diagnostic_bag())
            {
                std::cout << std::format("{} @ {}-{} ", diagnostic_type_name(diag.type()), diag.start_index(), diag.end_index());
                for (const auto& arg : diag.arguments())
                    std::cout << std::quoted(arg) << ' ';
                std::cout << '\n';
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
        auto parser = parse(std::move(a));
        auto parse = parser.parse_all();
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