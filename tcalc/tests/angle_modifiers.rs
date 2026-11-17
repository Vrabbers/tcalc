use tcalc::{
    evaluator::{AngleUnit, CancellationToken, Evaluator, Real, eval_result::EvalValue},
    lexer::Lexer,
    parser::Parser,
};

fn evaluate(evaluator: &Evaluator<Real>, source: &str) -> Real {
    let mut parser = Parser::new(Lexer::new(source.to_string(), true));
    let statement = parser.next().unwrap();
    assert!(parser.at_end(), "{source}");
    assert!(parser.diagnostic_bag().is_empty(), "{source}");
    match evaluator
        .evaluate(&statement, CancellationToken::default())
        .unwrap()
    {
        EvalValue::Numeric(value) => value,
        value => panic!("expected a number for {source}, got {value:?}"),
    }
}

#[test]
fn angle_modifiers_respect_the_active_mode() {
    for (mode, half_turn) in [
        (AngleUnit::Degrees, "180"),
        (AngleUnit::Radians, "pi"),
        (AngleUnit::Gradians, "200"),
    ] {
        let mut evaluator = Evaluator::new();
        evaluator.set_angle_unit(mode);

        for (source, expected) in [
            ("180deg", half_turn),
            ("180°", half_turn),
            ("pi rad", half_turn),
            ("200grad", half_turn),
            ("sin(90deg)", "1"),
            ("sin(90°)", "1"),
            ("sin((pi/2)rad)", "1"),
            ("sin(100grad)", "1"),
            ("cos(180deg)", "-1"),
            ("cos(pi rad)", "-1"),
            ("cos(200grad)", "-1"),
            ("tan(45deg)", "1"),
            ("tan((pi/4)rad)", "1"),
            ("tan(50grad)", "1"),
        ] {
            let actual = evaluate(&evaluator, source);
            let expected = evaluate(&evaluator, expected);
            assert!(
                actual.definitely_equals(&expected).unwrap(),
                "{source} in {mode} mode: {actual} != {expected}"
            );
        }
    }
}
