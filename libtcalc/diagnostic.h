#pragma once
#include "source_span.h"
#include <memory>
#include <set>

enum class tcDiagnosticType
{
    BadNumberLiteral,
    BadCharacter,
    BadSymbol
};

const char* tcDiagnosticTypeName(tcDiagnosticType type);

class tcDiagnostic final
{
public:
    // Constructor makes a copy of the sourceSpan, so it can live longer than the token which owns it.
    tcDiagnostic(const tcSourceSpan& src, tcDiagnosticType type) : _sourceSpan(std::make_unique<tcSourceSpan>(src)), _type(type)
    {}

    std::strong_ordering operator<=>(const tcDiagnostic& other) const
    {
        return sourceIndex() <=> other.sourceIndex();
    }

    [[nodiscard]]
    tcDiagnosticType type() const
    {
        return _type;
    }

    [[nodiscard]]
    tcSourcePosition position() const
    {
        return _sourceSpan->position();
    }

    [[nodiscard]]
    const tcSourceSpan& span() const
    {
        return *_sourceSpan;
    }

    [[nodiscard]]
    std::string_view sourceStr() const
    {
        return _sourceSpan->sourceStr();
    }

    [[nodiscard]]
    std::size_t sourceIndex() const
    {
        return _sourceSpan->sourceIndex();
    }

private:
    std::unique_ptr<tcSourceSpan> _sourceSpan;

    tcDiagnosticType _type;
};
