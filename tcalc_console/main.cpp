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

/*
static void show_arith(const tcalc::arithmetic_expression& arith)
{
    for (const auto& op : arith.tokens)
    {
        std::cout << op_to_string(op) << ' ';
    }
}
*/

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

        auto p = parse(std::move(input));
        std::vector<tcalc::expression> parse = p.parse_all();
        tcalc::evaluator eval{64};
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
            continue;
        }
        else
        {
            for (const auto& expr : parse)
            {
                if (const auto* arith = std::get_if<tcalc::arithmetic_expression>(&expr))
                {
                    auto before = std::chrono::steady_clock::now();
                    auto res = eval.evaluate_arithmetic(*arith);
                    std::chrono::duration<double, std::milli> time{std::chrono::steady_clock::now() - before};

                    if (res.is_error())
                    {
                        std::cout << "error " << tcalc::eval_error_type_name(res.error().type)
                            << " (" << res.error().position.start_index << ", " << res.error().position.end_index << ")";
                        break;
                    }
                    else
                    {
                        std::cout << res.value().string();
                    }
                    std::cout << " took " << time << '\n';
                }
                else if (const auto* asgn = std::get_if<tcalc::assignment_expression>(&expr))
                {
                    auto before = std::chrono::steady_clock::now();
                    auto res = eval.evaluate_assignment(*asgn);
                    std::chrono::duration<double, std::milli> time{std::chrono::steady_clock::now() - before};

                    if (res.is_error())
                    {
                        std::cout << "error " << tcalc::eval_error_type_name(res.error().type)
                            << " (" << res.error().position.start_index << ", " << res.error().position.end_index << ")";
                        break;
                    }
                }
                else if (const auto* blnexp = std::get_if<tcalc::boolean_expression>(&expr))
                {
                    auto before = std::chrono::steady_clock::now();
                    auto res = eval.evaluate_boolean(*blnexp);
                    std::chrono::duration<double, std::milli> time{std::chrono::steady_clock::now() - before};

                    if (res.is_error())
                    {
                        std::cout << "error " << tcalc::eval_error_type_name(res.error().type)
                            << " (" << res.error().position.start_index << ", " << res.error().position.end_index << ")";
                    }
                    else
                    {
                        std::cout << std::boolalpha << res.value();
                    }
                    std::cout << " took " << time << '\n';
                }
                else if (const auto* fndef = std::get_if<tcalc::func_def_expression>(&expr))
                {
                    auto before = std::chrono::steady_clock::now();
                    auto res = eval.evaluate_fn_def(*fndef);
                    std::chrono::duration<double, std::milli> time{std::chrono::steady_clock::now() - before};

                    if (res.is_error())
                    {
                        std::cout << "error " << tcalc::eval_error_type_name(res.error().type)
                            << " (" << res.error().position.start_index << ", " << res.error().position.end_index << ")";
                            break;
                    }
                }
            }
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