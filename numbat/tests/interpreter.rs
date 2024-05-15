mod common;

use common::get_test_context;

use insta::assert_snapshot;
use numbat::markup::{Formatter, PlainTextFormatter};
use numbat::resolver::CodeSource;
use numbat::NumbatError;
use numbat::{pretty_print::PrettyPrint, Context, InterpreterResult};

#[track_caller]
fn expect_output_with_context(ctx: &mut Context, code: &str, expected_output: impl AsRef<str>) {
    let expected_output = expected_output.as_ref();
    println!("Expecting output '{expected_output}' for code '{code}'");
    if let InterpreterResult::Value(val) = ctx.interpret(code, CodeSource::Internal).unwrap().1 {
        let fmt = PlainTextFormatter {};

        let actual_output = fmt.format(&val.pretty_print(), false);
        assert_eq!(actual_output.trim(), expected_output);
    } else {
        panic!();
    }
}

#[track_caller]
fn fail(code: &str) -> NumbatError {
    let mut ctx = get_test_context();
    let ret = ctx.interpret(code, CodeSource::Internal);
    match ret {
        Err(e) => e,
        Ok((_stmts, ret)) => {
            if let InterpreterResult::Value(val) = ret {
                let fmt = PlainTextFormatter {};
                let output = fmt.format(&val.pretty_print(), false);
                panic!("was supposed to fail but instead got:\n{}", output.trim())
            } else {
                panic!("was supposed to fail but instead got:\n{:?}", ret)
            }
        }
    }
}

#[track_caller]
fn expect_output(code: &str, expected_output: impl AsRef<str>) {
    let mut ctx = get_test_context();
    expect_output_with_context(&mut ctx, code, expected_output)
}

#[track_caller]
fn expect_failure(code: &str, msg_part: &str) {
    let mut ctx = get_test_context();
    if let Err(e) = ctx.interpret(code, CodeSource::Internal) {
        let error_message = e.to_string();
        println!("{}", error_message);
        assert!(
            error_message.contains(msg_part),
            "Expected '{msg_part}' but got '{error_message}'"
        );
    } else {
        panic!("Expected an error but but instead {code} did not fail");
    }
}

#[track_caller]
fn get_error_message(code: &str) -> String {
    let mut ctx = get_test_context();
    if let Err(e) = ctx.interpret(code, CodeSource::Internal) {
        e.to_string()
    } else {
        panic!();
    }
}

#[test]
fn simple_value() {
    expect_output("0", "0");
    expect_output("0_0", "0");
    expect_output("0_0.0_0", "0");
    expect_output(".0", "0");
    expect_failure("_.0", "Unexpected character in identifier: '.'");
    expect_output(".0_0", "0");
    expect_failure(".0_", "Unexpected character in number literal: '_'");

    expect_output("0b0", "0");
    expect_output("0b01", "1");
    expect_output("0b0_0", "0");
    expect_failure("0b012", "Expected base-2 digit");
    expect_failure("0b", "Expected base-2 digit");
    expect_failure("0b_", "Expected base-2 digit");
    expect_failure("0b_0", "Expected base-2 digit");
    expect_failure("0b0_", "Expected base-2 digit");
    expect_failure("0b0.0", "Expected base-2 digit");

    expect_output("0o0", "0");
    expect_output("0o01234567", "342_391");
    expect_output("0o0_0", "0");
    expect_failure("0o012345678", "Expected base-8 digit");
    expect_failure("0o", "Expected base-8 digit");
    expect_failure("0o_", "Expected base-8 digit");
    expect_failure("0o_0", "Expected base-8 digit");
    expect_failure("0o0_", "Expected base-8 digit");
    expect_failure("0o0.0", "Expected base-8 digit");

    expect_output("0x0", "0");
    expect_output("0x0123456789abcdef", "8.19855e+16");
    expect_output("0x0_0", "0");
    expect_failure("0x0123456789abcdefg", "Expected base-16 digit");
    expect_failure("0x", "Expected base-16 digit");
    expect_failure("0x_", "Expected base-16 digit");
    expect_failure("0x_0", "Expected base-16 digit");
    expect_failure("0x0_", "Expected base-16 digit");
    expect_failure("0x0.0", "Expected base-16 digit");

    expect_output("NaN", "NaN");
    expect_output("inf", "inf");
}

#[test]
fn test_factorial() {
    expect_output("0!", "1");
    expect_output("4!", "24");
    expect_output("4.0!", "24");
    expect_output("4 !", "24");
    expect_output(" 4 !", "24");
    expect_output("(4)!", "24");
    expect_output("3!^3", "216");
    // Not supported, at least for now.
    // expect_output("3!³", "216");
    expect_output("(3!)^3", "216");
    expect_output("3^3!", "729");
    expect_output("-5!", "-120");
    expect_output("-(5!)", "-120");
    expect_output("-(5)!", "-120");

    expect_failure(
        "(-1)!",
        "Expected factorial argument to be a non-negative integer",
    );
    expect_failure(
        "1.5!",
        "Expected factorial argument to be a finite integer number",
    );
    expect_failure(
        "(-1.5)!",
        "Expected factorial argument to be a non-negative integer",
    );
    expect_failure(
        "(2m)!",
        "Argument of factorial needs to be dimensionless (got Length).",
    );
}

#[test]
fn test_exponentiation() {
    expect_output("3²*2", "18");
    expect_output("3² 2", "18");
    expect_output("3²·2", "18");
    expect_output("3³*2", "54");
    expect_output("3³(2)", "54");
    expect_output("(1+2)²", "9");
    expect_output("2²pi", "12.5664");
    expect_output("2² pi", "12.5664");
    expect_output("2²·pi", "12.5664");
    expect_output("5m² to cm·m", "500 cm·m");
    expect_output("2⁵", "32");
    expect_output("-4¹", "-4");
    expect_output("2⁻¹", "0.5");
    expect_output("2⁻²", "0.25");
    expect_output("10⁻⁵", "0.00001");
}

#[test]
fn test_conversions() {
    expect_output("2in to cm", "5.08 cm");
    expect_output("5m^2 -> m*cm", "500 m·cm");
    expect_output("5m^2 -> cm*m", "500 cm·m");
    expect_output("1 kB / 10 ms -> MB/s", "0.1 MB/s");
    expect_output("55! / (6! (55 - 6)!) -> million", "28.9897 million");
}

#[test]
fn test_implicit_conversion() {
    let mut ctx = get_test_context();

    let _ = ctx.interpret("let x = 5 m", CodeSource::Internal).unwrap();

    expect_output_with_context(&mut ctx, "x", "5 m");
    expect_output_with_context(&mut ctx, "2x", "10 m");
    expect_output_with_context(&mut ctx, "2 x", "10 m");
    expect_output_with_context(&mut ctx, "x x", "25 m²");
    expect_output_with_context(&mut ctx, "x²", "25 m²");

    expect_failure("x2", "Unknown identifier 'x2'");
}

#[test]
fn test_reset_after_runtime_error() {
    let mut ctx = get_test_context();

    let _ = ctx.interpret("let x = 1", CodeSource::Internal).unwrap();
    let res = ctx.interpret("1/0", CodeSource::Internal);

    assert!(res.is_err());

    expect_output_with_context(&mut ctx, "x", "1");
}

#[test]
fn test_function_inverses() {
    expect_output("sin(asin(0.1234))", "0.1234");
    expect_output("cos(acos(0.1234))", "0.1234");
    expect_output("tan(atan(0.1234))", "0.1234");
    expect_output("sinh(asinh(0.1234))", "0.1234");
    expect_output("cosh(acosh(1.1234))", "1.1234");
    expect_output("tanh(atanh(0.1234))", "0.1234");
    expect_output("log(exp(0.1234))", "0.1234");
    expect_output("log10(10^0.1234)", "0.1234");
    expect_output("log2(2^0.1234)", "0.1234");
    expect_output("sqr(sqrt(0.1234))", "0.1234");

    expect_output("asin(sin(0.1234))", "0.1234");
    expect_output("acos(cos(0.1234))", "0.1234");
    expect_output("atan(tan(0.1234))", "0.1234");
    expect_output("asinh(sinh(0.1234))", "0.1234");
    expect_output("acosh(cosh(1.1234))", "1.1234");
    expect_output("atanh(tanh(0.1234))", "0.1234");
    expect_output("exp(log(0.1234))", "0.1234");
    expect_output("10^(log10(0.1234))", "0.1234");
    expect_output("2^(log2(0.1234))", "0.1234");
    expect_output("sqrt(sqr(0.1234))", "0.1234");
}

#[test]
fn test_algebra() {
    let mut ctx = get_test_context();
    let _ = ctx
        .interpret("use extra::algebra", CodeSource::Internal)
        .unwrap();
    expect_output_with_context(&mut ctx, "quadratic_equation(1, 0, -1)", "x₁ = 1; x₂ = -1");
    expect_output_with_context(&mut ctx, "quadratic_equation(0, 9, 3)", "x = -0.333333");
    expect_output_with_context(&mut ctx, "quadratic_equation(0, 0, 1)", "no solution");
    expect_output_with_context(&mut ctx, "quadratic_equation(9, -126, 441)", "x = 7");
    expect_output_with_context(&mut ctx, "quadratic_equation(1, -2, 1)", "x = 1");
    expect_output_with_context(&mut ctx, "quadratic_equation(0, 1, 1)", "x = -1");
    expect_output_with_context(&mut ctx, "quadratic_equation(1, 0, 0)", "x = 0");
    expect_output_with_context(
        &mut ctx,
        "quadratic_equation(0, 0, 0)",
        "infinitely many solutions",
    );
    expect_output_with_context(
        &mut ctx,
        "quadratic_equation(1, 1, 1)",
        "no real-valued solution",
    );
}

#[test]
fn test_math() {
    expect_output("sin(90°)", "1");
    expect_output("sin(30°)", "0.5");
    expect_output("sin(pi/2)", "1");

    expect_output("atan2(10, 0) / (pi / 2)", "1");
    expect_output("atan2(100 cm, 1 m) / (pi / 4)", "1");
    expect_failure(
        "atan2(100 cm, 1 m²)",
        "parameter type: Length\n argument type: Length²",
    );

    expect_output("mod(5, 3)", "2");
    expect_output("mod(-1, 4)", "3");
    expect_output("mod(8 cm, 5 cm)", "3 cm");
    expect_output("mod(235 cm, 1 m)", "35 cm");
    expect_output("mod(2 m, 7 cm)", "0.04 m");
    expect_failure(
        "mod(8 m, 5 s)",
        "parameter type: Length\n argument type: Time",
    )
}

#[test]
fn test_incompatible_dimension_errors() {
    assert_snapshot!(
        get_error_message("kg m / s^2 + kg m^2"),
        @r###"
     left hand side: Length  × Mass × Time⁻²    [= Force]
    right hand side: Length² × Mass             [= MomentOfInertia]
    "###
    );

    assert_snapshot!(
        get_error_message("1 + m"),
        @r###"
     left hand side: Scalar    [= Angle, Scalar, SolidAngle]
    right hand side: Length

    Suggested fix: divide the expression on the right hand side by a `Length` factor
    "###
    );

    assert_snapshot!(
        get_error_message("m / s + K A"),
        @r###"
     left hand side: Length / Time            [= Velocity]
    right hand side: Current × Temperature
    "###
    );

    assert_snapshot!(
        get_error_message("m + 1 / m"),
        @r###"
     left hand side: Length
    right hand side: Length⁻¹    [= Wavenumber]

    Suggested fix: invert the expression on the right hand side
    "###
    );

    assert_snapshot!(
        get_error_message("kW -> J"),
        @r###"
     left hand side: Length² × Mass × Time⁻³    [= Power]
    right hand side: Length² × Mass × Time⁻²    [= Energy, Torque]

    Suggested fix: divide the expression on the right hand side by a `Time` factor
    "###
    );

    assert_snapshot!(
        get_error_message("sin(1 meter)"),
        @r###"
    parameter type: Scalar    [= Angle, Scalar, SolidAngle]
     argument type: Length

    Suggested fix: divide the function argument by a `Length` factor
    "###
    );

    assert_snapshot!(
        get_error_message("let x: Acceleration = 4 m / s"),
        @r###"
    specified dimension: Length × Time⁻²    [= Acceleration]
       actual dimension: Length × Time⁻¹    [= Velocity]

    Suggested fix: divide the right hand side expression by a `Time` factor
    "###
    );

    assert_snapshot!(
        get_error_message("unit x: Acceleration = 4 m / s"),
        @r###"
    specified dimension: Length × Time⁻²    [= Acceleration]
       actual dimension: Length × Time⁻¹    [= Velocity]

    Suggested fix: divide the right hand side expression by a `Time` factor
    "###
    );

    assert_snapshot!(
        get_error_message("fn acceleration(length: Length, time: Time) -> Acceleration = length / time"),
        @r###"
    specified return type: Length × Time⁻²    [= Acceleration]
       actual return type: Length × Time⁻¹    [= Velocity]

    Suggested fix: divide the expression in the function body by a `Time` factor
    "###
    );
}

#[test]
fn test_temperature_conversions() {
    expect_output("from_celsius(11.5)", "284.65 K");
    expect_output("from_fahrenheit(89.3)", "304.983 K");
    expect_output("0 K -> celsius", "-273.15");
    expect_output("fahrenheit(30 K)", "-405.67");
    expect_output("from_celsius(100) -> celsius", "100");
    expect_output("from_fahrenheit(100) -> fahrenheit", "100.0");
    expect_output("from_celsius(123 K -> celsius)", "123 K");
    expect_output("from_fahrenheit(123 K -> fahrenheit)", "123 K");

    expect_output("-40 -> from_fahrenheit -> celsius", "-40");
}

#[test]
fn test_other_functions() {
    expect_output("sqrt(4)", "2");
    expect_output("log10(100000)", "5");
    expect_output("log(e^15)", "15");
    expect_output("ln(e^15)", "15");
    expect_output("ceil(3.1)", "4");
    expect_output("floor(3.9)", "3");
    expect_output("round(3.9)", "4");
    expect_output("round(3.1)", "3");
    expect_output("is_nan(NaN)", "true");
    expect_output("is_nan(NaN cm)", "true");
    expect_output("is_nan(ln(-1))", "true");
    expect_output("is_nan(1)", "false");
    expect_output("is_infinite(inf)", "true");
    expect_output("is_infinite(-inf)", "true");
    expect_output("is_infinite(1)", "false");
}

#[test]
fn test_last_result_identifier() {
    let mut ctx = get_test_context();

    let _ = ctx.interpret("2 + 3", CodeSource::Internal).unwrap();
    expect_output_with_context(&mut ctx, "ans", "5");

    let _ = ctx.interpret("1 + 2", CodeSource::Internal).unwrap();
    expect_output_with_context(&mut ctx, "_", "3");
}

#[test]
fn test_misc_examples() {
    expect_output("1920/16*9", "1080");
    expect_output("2^32", "4_294_967_296");
    expect_output("sqrt(1.4^2 + 1.5^2) * cos(pi/3)^2", "0.512957");

    expect_output("2min + 30s", "2.5 min");
    expect_output("2min + 30s -> sec", "150 s");
    expect_output("4/3 * pi * (6000km)³", "9.04779e+11 km³");
    expect_output("40kg * 9.8m/s^2 * 150cm", "588 kg·m²/s²");
    expect_output("sin(30°)", "0.5");

    expect_output("60mph -> m/s", "26.8224 m/s");
    expect_output("240km/day -> km/h", "10 km/h");
    expect_output("1mrad -> °", "0.0572958°");
    expect_output("52weeks -> days", "364 day");
    expect_output("5in + 2ft -> cm", "73.66 cm");
    expect_output("atan(30cm / 2m) -> deg", "8.53077°");
    expect_output("6Mbit/s * 1.5h -> GB", "4.05 GB");
    expect_output("6Mbit/s * 1.5h -> GiB", "3.77186 GiB");

    expect_output("3m/4m", "0.75");
    expect_output("4/2*2", "4");
    expect_output("1/2 Hz -> s", "0.5 s");
}

#[test]
fn test_bohr_radius_regression() {
    // Make sure that the unit is 'm', and not 'F·J²/(C²·kg·m·Hz²)', like we had before
    expect_output("bohr_radius", "5.29177e-11 m");
}

#[test]
fn test_full_simplify() {
    expect_output("5 cm/m", "0.05");
    expect_output("hour/second", "3600");

    expect_output("5 to cm/m", "500 cm/m");
    expect_output(
        "fn f(x: Scalar) -> Scalar = x to cm/m
             f(5)",
        "500 cm/m",
    );

    expect_output("1 Wh/W", "1 Wh/W"); // This output is not great (and should be improved). But we keep this as a regression test for a bug in previous versions.

    expect_output("1 × (m/s)^2/(m/s)", "1 m/s");
}

#[test]
fn test_prefixes() {
    expect_output("hertz second", "1");
    expect_output("kilohertz millisecond", "1");
    expect_output("megahertz microsecond", "1");
    expect_output("gigahertz nanosecond", "1");
    expect_output("terahertz picosecond", "1");
    expect_output("petahertz femtosecond", "1");
    expect_output("exahertz attosecond", "1");
    expect_output("zettahertz zeptosecond", "1");
    expect_output("yottahertz yoctosecond", "1");
    expect_output("ronnahertz rontosecond", "1");
    expect_output("quettahertz quectosecond", "1");
}

#[test]
fn test_parse_errors() {
    expect_failure(
        "3kg+",
        "Expected one of: number, identifier, parenthesized expression, struct instantiation",
    );
    expect_failure("let print=2", "Expected identifier after 'let' keyword");
    expect_failure(
        "fn print(x: Scalar) = 1",
        "Expected identifier after 'fn' keyword",
    );
}

#[test]
fn test_name_clash_errors() {
    expect_failure("let kg=2", "Identifier is already in use: 'kg'");
    expect_failure("fn kg(x: Scalar) = 1", "Identifier is already in use: 'kg'");
    expect_failure("fn _()=0", "Reserved identifier");
}

#[test]
fn test_type_check_errors() {
    expect_failure("foo", "Unknown identifier 'foo'");

    expect_failure(
        "let sin=2",
        "Identifier is already in use by the foreign function: 'sin'",
    );
    expect_failure(
        "fn pi() = 1",
        "Identifier is already in use by the constant: 'pi'",
    );
    expect_failure(
        "fn sin(x)=0",
        "Identifier is already in use by the foreign function: 'sin'",
    );
}

#[test]
fn test_runtime_errors() {
    expect_failure("1/0", "Division by zero");
}

#[test]
fn test_comparisons() {
    expect_output("2 < 3", "true");
    expect_output("2 m < 3 m", "true");
    expect_output("20 cm < 3 m", "true");
    expect_output("2 m < 100 cm", "false");

    expect_output("2 > 3", "false");
    expect_output("2 m > 3 m", "false");
    expect_output("20 cm > 3 m", "false");
    expect_output("2 m > 100 cm", "true");

    expect_output("2 <= 2", "true");
    expect_output("2.1 <= 2", "false");

    expect_output("2 >= 2", "true");
    expect_output("2 >= 2.1", "false");

    expect_output("200 cm == 2 m", "true");
    expect_output("201 cm == 2 m", "false");

    expect_output("200 cm != 2 m", "false");
    expect_output("201 cm != 2 m", "true");
}

#[test]
fn test_logical() {
    // negation
    expect_output("!true", "false");
    expect_output("!false", "true");

    // or
    expect_output("true || false", "true");
    expect_output("false || false", "false");

    // and
    expect_output("true && false", "false");
    expect_output("true && true", "true");

    // priority
    expect_output("false || true && false", "false");
    expect_output("false || true && !false", "true");

    // Errors
    insta::assert_display_snapshot!(fail("1 || 2"), @"Expected boolean value");
    insta::assert_display_snapshot!(fail("true || 2"), @"Expected boolean value");
    insta::assert_display_snapshot!(fail("1 || true"), @"Expected boolean value");
    insta::assert_display_snapshot!(fail("1 && 2"), @"Expected boolean value");
    insta::assert_display_snapshot!(fail("true && 2"), @"Expected boolean value");
    insta::assert_display_snapshot!(fail("1 && true"), @"Expected boolean value");
    insta::assert_display_snapshot!(fail("!1"), @"Expected boolean value");
    insta::assert_display_snapshot!(fail("!1 || true"), @"Expected boolean value");
}

#[test]
fn test_conditionals() {
    expect_output("if 1 < 2 then 3 else 4", "3");
    expect_output("if 4 < 3 then 2 else 1", "1");
    expect_output(
        "if 4 > 3 then \"four is larger!\" else \"four is not larger!\"",
        "four is larger!",
    );
}

#[test]
fn test_string_interpolation() {
    expect_output("\"pi = {pi}!\"", "pi = 3.14159!");
    expect_output("\"1 + 2 = {1 + 2}\"", "1 + 2 = 3");

    expect_output("\"{0.2:0.5}\"", "0.20000");
    expect_output("\"pi ~= {pi:.3}\"", "pi ~= 3.142");
    expect_output(
        "\"both {pi:.3} and {e} are irrational and transcendental numbers\"",
        "both 3.142 and 2.71828 are irrational and transcendental numbers",
    );
    expect_output(
        "
        let str = \"1234\"
        \"{str:0.2}\"
        ",
        "12",
    );

    expect_output("\"{1_000_300:+.3}\"", "+1000300.000");

    expect_output(
        "
        let str = \"1234\"
        \"a {str:^10} b\"
        ",
        "a    1234    b",
    );

    // Doesn't work at the moment, as `strfmt` expects `i64`'s for `#x`, but Numbat deals with `f64`'s
    // internally
    //expect_output("\"{31:#x}\"", "0x1f")

    expect_failure(
        "\"{200:x}\"",
        "Incorrect type for format specifiers: Unknown format code 'x' for type",
    );
    expect_failure(
        "\"{200:.}\"",
        "Invalid format specifiers: Format specifier missing precision",
    );

    expect_failure(
        "
        let str = \"1234\"
        \"{str:.3f}\"
        ",
        "Incorrect type for format specifiers: Unknown format code Some('f') for object of type 'str'",
    );
}

#[test]
fn test_overwrite_regular_function() {
    expect_output(
        "
        fn f(x)=0
        fn f(x)=1
        f(2)",
        "1",
    );
}

#[test]
fn test_overwrite_inner_function() {
    expect_output(
        "
        fn inner() = 0
        fn outer() = inner()

        fn inner(x) = 1
        outer()
        ",
        "0",
    );
}

#[test]
fn test_override_constants() {
    expect_output("let x = 1\nlet x = 2\nx", "2");
    expect_output("let pi = 4\npi", "4");
}

#[test]
fn test_overwrite_captured_constant() {
    expect_output(
        "
        let x = 1
        fn f() = sin(x)

        let x = 1 m
        f()
        ",
        "0.841471",
    );
}

#[test]
fn test_pretty_print_prefixes() {
    expect_output("1 megabarn", "1 megabarn");
}

#[test]
fn test_full_simplify_for_function_calls() {
    expect_output("floor(1.2 hours / hour)", "1");
}

#[test]
fn test_datetime_runtime_errors() {
    expect_failure("datetime(\"2000-01-99\")", "Unrecognized datetime format");
    expect_failure("now() -> tz(\"Europe/NonExisting\")", "Unknown timezone");
    expect_failure(
        "date(\"2000-01-01\") + 1e100 years",
        "Exceeded maximum size for time durations",
    );
    expect_failure(
        "date(\"2000-01-01\") + 100000000 years",
        "DateTime out of range",
    );
    expect_failure(
        "format_datetime(\"%Y-%m-%dT%H%:M\", now())",
        "Error in datetime format",
    )
}

#[test]
fn test_user_errors() {
    expect_failure("error(\"test\")", "User error: test");

    // Make sure that the never type (!) can be used in all contexts
    expect_failure("- error(\"test\")", "User error: test");
    expect_failure("1 + error(\"test\")", "User error: test");
    expect_failure("1 m + error(\"test\")", "User error: test");
    expect_failure("if 3 < 2 then 2 m else error(\"test\")", "User error: test");
}
