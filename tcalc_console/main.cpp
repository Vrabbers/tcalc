#include <array>
#include <iostream>
#include <format>
#include <random>
#include <cstring>
#include <iomanip>
#include <chrono>

#include "tc_evaluator.h"
#include "tc_expression.h"
#include "tc_lexer.h"
#include "tc_number.h"
#include "tc_parser.h"

#ifdef _WIN32
#include <windows.h>
#endif

#ifdef _MSC_VER
#pragma execution_character_set("utf-8")
#endif

constexpr long prec = 64;

static tcalc::parser parse(std::string&& str)
{
    tcalc::lexer lexer{std::move(str), true};
    tcalc::parser parser{std::move(lexer), prec};

    return parser;
}
/*
static void show_arith(const tcalc::arithmetic_expression& arith)
{
    for (const auto& op : arith.tokens)
    {
        std::cout << op_to_string(op) << ' ';
    }
}
*/
static void eval(tcalc::evaluator& eval, std::string&& input, bool show)
{
    auto p = parse(std::move(input));
    std::vector<tcalc::expression> parse = p.parse_all();
    if (!p.diagnostic_bag().empty())
    {
        if (!show)
            return;

        std::cout << "\n\x1b[31mdiagnostics:\n";
        for (const auto& diag : p.diagnostic_bag())
        {
            std::cout << std::format("{} @ {}-{} ", diagnostic_type_name(diag.type()), diag.start_index(), diag.end_index());
            for (const auto& arg : diag.arguments())
                std::cout << std::quoted(arg) << ' ';
            std::cout << '\n';
        }
        std::cout << "\x1b[0m";
        return;
    }


    for (const auto& expr : parse)
    {
        auto before = std::chrono::steady_clock::now();
        auto res = eval.evaluate(expr);
        std::chrono::duration<double, std::milli> time{std::chrono::steady_clock::now() - before};

        if (!show)
            continue;

        if (res.is_error())
        {
            std::cout << "error " << tcalc::eval_error_type_name(res.error().type)
                << " (" << res.error().position.start_index << ", " << res.error().position.end_index << ")";
            break;
        }

        if (const auto* num = std::get_if<tcalc::number>(&res.value()))
        {
            std::cout << num->string() << ' ' << num->dbg_string() << '\n';
            std::cout << "normal:     " << num->string(0, tcalc::number_format::normal) << '\n';
            std::cout << "fixed:      " << num->string(0, tcalc::number_format::fixed_point) << '\n';
            std::cout << "scientific: " << num->string(0, tcalc::number_format::scientific) << '\n';
        }
        else if (const auto* boolean = std::get_if<bool>(&res.value()))
        {
            std::cout << std::boolalpha << *boolean;
        }
        else
        {
            std::cout << "empty result expression";
        }
        std::cout << " took " << time << '\n';
        eval.commit_result(res.value());
    }
}

static void interactive()
{
    std::cout << "tcalc console\n";

    tcalc::evaluator evaluator{prec};

    while (true)
    {
        std::string input;
        std::cout << "> ";
        std::getline(std::cin, input);

        if (input == "quit")
            return;

        eval(evaluator, std::move(input), true);

        std::cout << '\n';
    }
    
}

static void fuzz(const int times)
{
    std::random_device rd;
    std::default_random_engine rand(rd());
    std::uniform_int_distribution rdist(0, 255);
    std::array<char, 512> buf{};
    tcalc::evaluator evaluator{prec};

    for (int i = 0; i < times; i++)
    {
        for (char& j : buf)
            j = static_cast<char>(rdist(rand));
        std::cout << "Fuzz #" << i + 1 << '\n';
        std::string a{buf.cbegin(), buf.cend()};
        eval(evaluator, std::move(a), false);
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