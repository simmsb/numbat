use super::macros::*;
use super::Args;
use super::Result;

use crate::quantity::Quantity;
use crate::value::Value;

pub fn mod_(mut args: Args) -> Result<Value> {
    let x = quantity_arg!(args);
    let y = quantity_arg!(args);

    let x_value = x.unsafe_value().clone();
    let y_value = y.convert_to(x.unit()).unwrap().unsafe_value().clone();

    return_quantity!(x_value.rem_euclid(y_value), x.unit().clone())
}

// A simple math function with signature 'Fn[(Scalar) -> Scalar]'
macro_rules! simple_scalar_math_function {
    ($name:ident, $op:ident) => {
        pub fn $name(mut args: Args) -> Result<Value> {
            let value = scalar_arg!(args);
            return_scalar!(crate::number::Number::new(value.value.$op()))
        }
    };
    ($name:ident, $op:ident, fallible) => {
        pub fn $name(mut args: Args) -> Result<Value> {
            let value = scalar_arg!(args);
            return_scalar!(crate::number::Number::new(value.value.$op()?))
        }
    };
    ($name:ident, $op:ident, f64) => {
        pub fn $name(mut args: Args) -> Result<Value> {
            let value = scalar_arg!(args).to_f64();
            return_scalar!(crate::number::Number::from_f64(value.$op()))
        }
    };
}

pub fn abs(mut args: Args) -> Result<Value> {
    let arg = quantity_arg!(args);
    return_quantity!(arg.unsafe_value().clone().abs(), arg.unit().clone())
}

simple_scalar_math_function!(round, round, f64);
simple_scalar_math_function!(floor, floor, f64);
simple_scalar_math_function!(ceil, ceil, f64);
simple_scalar_math_function!(trunc, trunc, f64);
simple_scalar_math_function!(fract, fract, f64);

simple_scalar_math_function!(sin, sin);
simple_scalar_math_function!(cos, cos);
simple_scalar_math_function!(tan, tan, fallible);
simple_scalar_math_function!(asin, asin, f64);
simple_scalar_math_function!(acos, acos, f64);
simple_scalar_math_function!(atan, atan, f64);

pub fn atan2(mut args: Args) -> Result<Value> {
    let y = quantity_arg!(args);
    let x = quantity_arg!(args);

    let y_value = y.unsafe_value().clone().to_f64();
    let x_value = x.convert_to(y.unit()).unwrap().unsafe_value().clone().to_f64();

    return_scalar!(crate::number::Number::from_f64(y_value.atan2(x_value)))
}

simple_scalar_math_function!(sinh, sinh, f64);
simple_scalar_math_function!(cosh, cosh, f64);
simple_scalar_math_function!(tanh, tanh, f64);
simple_scalar_math_function!(asinh, asinh, f64);
simple_scalar_math_function!(acosh, acosh, f64);
simple_scalar_math_function!(atanh, atanh, f64);
simple_scalar_math_function!(exp, exp, fallible);
simple_scalar_math_function!(ln, ln, fallible);
simple_scalar_math_function!(log10, log10, fallible);
simple_scalar_math_function!(log2, log2, f64);

pub fn gamma(mut args: Args) -> Result<Value> {
    let input = scalar_arg!(args).to_f64();

    return_scalar!(crate::number::Number::from_f64(crate::gamma::gamma(input)))
}

pub fn is_nan(mut args: Args) -> Result<Value> {
    let arg = quantity_arg!(args);

    return_boolean!(arg.unsafe_value().poison.map_or(false, |x| x.is_nan()))
}

pub fn is_infinite(mut args: Args) -> Result<Value> {
    let arg = quantity_arg!(args);

    return_boolean!(arg.unsafe_value().poison.map_or(false, |x| x.is_infinite()))
}

pub fn random(_args: Args) -> Result<Value> {
    return_scalar!(crate::number::Number::from_f64(rand::random::<f64>()))
}
