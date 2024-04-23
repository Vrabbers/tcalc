#pragma once

#include <string>
#include <memory>

#include "diagnostic.h"
#include "token.h"
#include "string_reader.h"

class tcLexer final
{
public:
    explicit tcLexer(std::string&& input, bool commaArgSeparator) : _sr(std::make_unique<tcStringReader>(std::move(input))),
                                                                    _commaArgumentSeparator(commaArgSeparator)
    {
    }

    explicit tcLexer(const char* input, bool commaArgSeparator) : _sr(std::make_unique<tcStringReader>(input)),
                                                                  _commaArgumentSeparator(commaArgSeparator)
    {
    }

    [[nodiscard]]
    const std::multiset<tcDiagnostic>& diagnosticBag() const
    {
        return *_diagnosticBag;
    }

    tcToken next();

private:
    tcToken flushToken(tcTokenKind);
    tcToken lexNumber();
    tcToken lexSuperscriptNumber();
    tcToken lexSymbol();
    tcToken lexWord();

    [[nodiscard]]
    inline char32_t decimalSeparator() const;
    [[nodiscard]]
    inline char32_t argSeparator() const;

    std::unique_ptr<std::multiset<tcDiagnostic>> _diagnosticBag = std::make_unique<std::multiset<tcDiagnostic>>();
    std::unique_ptr<tcStringReader> _sr;
    bool _commaArgumentSeparator;
};
