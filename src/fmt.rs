
#[cfg(test)]
mod tests {
    use std::num::{ParseFloatError, ParseIntError};
    use std::num::IntErrorKind::*;

    macro_rules! assert_err {
        ($actual:expr $(,)?) => {
            ::std::assert!($actual.is_err())
        };
        ($actual:expr, $err:ident $(,)?) => {
            ::std::assert!(::std::matches!($actual, Err($err {..})))
        };
        ($actual:expr, $err:ident($kind:expr) $(,)?) => {
            ::std::assert!(::std::matches!($actual, Err(e @ $err {..}) if *e.kind() == $kind))
        };
    }

    #[test]
    fn parse_decimal_int() -> Result<(), ParseIntError> {
        let s: i32 = "123".parse()?;
        assert_eq!(s, 123);

        let s: i32 = "-123".parse()?;
        assert_eq!(s, -123);

        // It's not C octal...
        let s: i32 = "010".parse()?;
        assert_eq!(s, 10);

        // ...or Rust octal...
        assert_err!("0o10".parse::<i32>(), ParseIntError(InvalidDigit));

        // ...or hex
        assert_err!("0x10".parse::<i32>(), ParseIntError(InvalidDigit));

        // + is ok
        let s: i32 = "+123".parse()?;
        assert_eq!(s, 123);

        // No trailing garbage
        assert_err!("123a".parse::<i32>(), ParseIntError(InvalidDigit));

        // No double neg
        assert_err!("--123".parse::<i32>(), ParseIntError(InvalidDigit));

        // No leading space
        assert_err!(" 123".parse::<i32>(), ParseIntError(InvalidDigit));

        // No trailing space
        assert_err!("123 ".parse::<i32>(), ParseIntError(InvalidDigit));

        // ASCII-only
        assert_err!("１２３".parse::<i32>(), ParseIntError(InvalidDigit));

        // Too big
        assert_err!("2147483648".parse::<i32>(), ParseIntError(PosOverflow));
        assert_err!("-2147483649".parse::<i32>(), ParseIntError(NegOverflow));

        // No underscore
        assert_err!("123_000".parse::<i32>(), ParseIntError(InvalidDigit));

        // No comma
        assert_err!("123,000".parse::<i32>(), ParseIntError(InvalidDigit));

        // Not try_into
        // let s: i32 = "-123".try_into()?;
        // assert_eq!(s, -123);
        Ok(())
    }

    #[test]
    fn format_decimal_int() {
        let pos = 123;
        let neg = -123;
        assert_eq!(pos.to_string(), "123");
        assert_eq!(neg.to_string(), "-123");

        // Display
        assert_eq!(format!("{}", pos), "123");
        assert_eq!(format!("{}", neg), "-123");
        // Debug
        assert_eq!(format!("{pos:?}"), "123");
        assert_eq!(format!("{neg:?}"), "-123");
        // Sign
        assert_eq!(format!("{pos:+}"), "+123");
        assert_eq!(format!("{neg:+}"), "-123");

        // Left/right space-pad
        assert_eq!(format!("{pos:5}"), "  123");
        assert_eq!(format!("{neg:5}"), " -123");
        assert_eq!(format!("{pos:<5}"), "123  ");
        assert_eq!(format!("{neg:<5}"), "-123 ");
        assert_eq!(format!("{pos:<+5}"), "+123 ");
        assert_eq!(format!("{neg:<+5}"), "-123 ");
        assert_eq!(format!("{pos:^5}"), " 123 ");
        assert_eq!(format!("{neg:^5}"), "-123 ");
        assert_eq!(format!("{pos:>5}"), "  123");
        assert_eq!(format!("{neg:>5}"), " -123");

        // Left-only zero-pad (numeric)
        assert_eq!(format!("{pos:05}"), "00123");
        assert_eq!(format!("{neg:05}"), "-0123");
        assert_eq!(format!("{pos:<05}"), "00123");
        assert_eq!(format!("{neg:<05}"), "-0123");
        assert_eq!(format!("{pos:<+05}"), "+0123");
        assert_eq!(format!("{neg:<+05}"), "-0123");
        assert_eq!(format!("{pos:^05}"), "00123");
        assert_eq!(format!("{neg:^05}"), "-0123");
        assert_eq!(format!("{pos:>05}"), "00123");
        assert_eq!(format!("{neg:>05}"), "-0123");

        // ⚠️ Left-right zero-pad (non-numeric)
        assert_eq!(format!("{pos:0<5}"), "12300");
        assert_eq!(format!("{neg:0<5}"), "-1230");
        assert_eq!(format!("{pos:0<+5}"), "+1230");
        assert_eq!(format!("{neg:0<+5}"), "-1230");
        assert_eq!(format!("{pos:0^5}"), "01230");
        assert_eq!(format!("{neg:0^5}"), "-1230");
        assert_eq!(format!("{pos:0>5}"), "00123");
        assert_eq!(format!("{neg:0>5}"), "0-123");

        // Left-right any-pad
        assert_eq!(format!("{pos:🦀<5}"), "123🦀🦀");
        assert_eq!(format!("{neg:🦀<5}"), "-123🦀");
        assert_eq!(format!("{pos:🦀^5}"), "🦀123🦀");
        assert_eq!(format!("{neg:🦀^5}"), "-123🦀");
        assert_eq!(format!("{pos:🦀>5}"), "🦀🦀123");
        assert_eq!(format!("{neg:🦀>5}"), "🦀-123");

        // No grouping/localisation
        // assert_eq!(format!("{123000:,}"), "123,000");
        // No
        // assert_eq!(String::from(i), "123");
        // No
        // assert_eq!(String::try_from(i).unwrap(), "123");
    }

    #[test]
    fn parse_hex_int() -> Result<(), ParseIntError> {
        // Case-insensitive
        assert_eq!(i32::from_str_radix("7f", 16), Ok(0x7f));
        assert_eq!(i32::from_str_radix("7F", 16), Ok(0x7f));
        // No 0x prefix
        assert_err!(i32::from_str_radix("0x7F", 16), ParseIntError(InvalidDigit));

        assert_eq!(i32::from_str_radix("-1", 16), Ok(-1));
        // It's not -1_i32. Asymmetric with format {:x}
        assert_err!(i32::from_str_radix("ffffffff", 16), ParseIntError(PosOverflow));
        Ok(())
    }

    #[test]
    fn format_hex_int() {
        assert_eq!(format!("{:x}", 0x7f), "7f");
        assert_eq!(format!("{:X}", 0x7f), "7F");

        assert_eq!(format!("{:#x}", 0x7f), "0x7f");
        assert_eq!(format!("{:#06x}", 0x7f), "0x007f");

        assert_eq!(format!("{:<#6x}", 0x7f), "0x7f  ");
        assert_eq!(format!("{:^#6x}", 0x7f), " 0x7f ");
        assert_eq!(format!("{:>#6x}", 0x7f), "  0x7f");

        assert_eq!(format!("{:x}", -1), "ffffffff");
        assert_eq!(format!("{:010x}", -1), "00ffffffff");

        // Literal out of range for `i32`
        // assert_eq!(format!("{:x}", 0xffffffff_i32), "ffffffff");
    }

    #[test]
    fn parse_float_finite() -> Result<(), ParseFloatError> {
        // Integer
        let s: f32 = "123".parse()?;
        assert_eq!(s, 123.0);

        // Trailing dot
        let s: f32 = "123.".parse()?;
        assert_eq!(s, 123.0);

        // Leading dot
        let s: f32 = ".25".parse()?;
        assert_eq!(s, 0.25);

        // Negative leading dot
        let s: f32 = "-.25".parse()?;
        assert_eq!(s, -0.25);

        // Not dot
        assert_err!(".".parse::<f32>(), ParseFloatError);

        let s: f32 = "123.25".parse()?;
        assert_eq!(s, 123.25);

        let s: f32 = "-123.25".parse()?;
        assert_eq!(s, -123.25);

        // + is ok
        let s: f32 = "+123.25".parse()?;
        assert_eq!(s, 123.25);

        // e
        let s: f32 = "12.325e1".parse()?;
        assert_eq!(s, 123.25);

        let s: f32 = "12.325e+1".parse()?;
        assert_eq!(s, 123.25);

        let s: f32 = "1232.5e-1".parse()?;
        assert_eq!(s, 123.25);

        // No e dot
        assert_err!("12.325e+1.0".parse::<f32>(), ParseFloatError);

        // Big E
        let s: f32 = "12.325E1".parse()?;
        assert_eq!(s, 123.25);

        // No trailing e
        assert_err!("123.25e".parse::<f32>());

        // No leading e
        assert_err!("e1".parse::<f32>());
        Ok(())
    }

    #[test]
    fn parse_float_nonfinite() -> Result<(), ParseFloatError> {
        // nan
        let s = "nan".parse::<f32>()?;
        assert!(s.is_nan() && s.is_sign_positive());

        // nan case-insensitive
        let s = "nAn".parse::<f32>()?;
        assert!(s.is_nan() && s.is_sign_positive());

        // -nan
        let s = "-nan".parse::<f32>()?;
        assert!(s.is_nan() && s.is_sign_negative());

        // inf
        let s = "inf".parse::<f32>()?;
        assert!(s.is_infinite() && s.is_sign_positive());

        // infinity
        let s = "infinity".parse::<f32>()?;
        assert!(s.is_infinite() && s.is_sign_positive());

        // Not infin
        let s = "infin".parse::<f32>();
        assert_err!(s, ParseFloatError);

        // inf case-insensitive
        let s = "iNf".parse::<f32>()?;
        assert!(s.is_infinite() && s.is_sign_positive());

        // -inf
        let s = "-inf".parse::<f32>()?;
        assert!(s.is_infinite() && s.is_sign_negative());

        // Not Excel
        assert_err!("#N/A".parse::<f32>(), ParseFloatError);
        Ok(())
    }

    #[test]
    fn parse_float_zero() -> Result<(), ParseFloatError> {
        // Positive zero
        assert!("0".parse::<f32>()?.is_sign_positive());
        assert!("0.0".parse::<f32>()?.is_sign_positive());
        assert!("+0.0".parse::<f32>()?.is_sign_positive());

        // Negative zero
        assert!("-0".parse::<f32>()?.is_sign_negative());
        assert!("-0.0".parse::<f32>()?.is_sign_negative());

        Ok(())
    }

    #[test]
    fn parse_float_limits() -> Result<(), ParseFloatError> {
        // Slightly under denormal min, rounds up
        assert_eq!("0.8e-45".parse::<f32>(), Ok(1e-45));
        // Normal min +ve
        assert_eq!("1.17549435e-38".parse::<f32>(), Ok(f32::MIN_POSITIVE));
        // Too small, underflow to zero
        assert_eq!("1.40129846432e-46".parse::<f32>(), Ok(0.0));

        // Slightly over max, rounds down
        assert_eq!("340282346638528859811704183484516925441".parse::<f32>(), Ok(f32::MAX));
        // Too big
        assert_eq!("340282356800000000000000000000000000000".parse::<f32>(), Ok(f32::INFINITY));

        // Not try_into
        // let s: f32 = "-123.25".try_into()?;
        // assert_eq!(s, -123.25);
        Ok(())
    }

    #[test]
    fn format_float() {
        // Numbers below are exactly representable

        assert_eq!(128.25.to_string(), "128.25");
        assert_eq!((-128.25).to_string(), "-128.25");

        assert_eq!(format!("{}",     128.25), "128.25");
        assert_eq!(format!("{}",    -128.25), "-128.25");
        assert_eq!(format!("{:?}",   128.25), "128.25");
        assert_eq!(format!("{:?}",  -128.25), "-128.25");
        assert_eq!(format!("{:+}",   128.25), "+128.25");
        assert_eq!(format!("{:+}",  -128.25), "-128.25");
        assert_eq!(format!("{:+?}",  128.25), "+128.25");
        assert_eq!(format!("{:+?}", -128.25), "-128.25");

        assert_eq!(format!("{:10.3}",   128.25), "   128.250");
        assert_eq!(format!("{:10.3}",  -128.25), "  -128.250");
        assert_eq!(format!("{:+10.3}",  128.25), "  +128.250");
        assert_eq!(format!("{:+10.3}", -128.25), "  -128.250");

        // Round to nearest, ties to even
        assert_eq!(format!("{:.1}",  128.25), "128.2");
        assert_eq!(format!("{:.1}", -128.25), "-128.2");
        // Note different number: 128 point *seven* five
        assert_eq!(format!("{:.1}",  128.75), "128.8");
        assert_eq!(format!("{:.1}", -128.75), "-128.8");

        assert_eq!(format!("{:010.3e}",   128.25), "0001.282e2");
        assert_eq!(format!("{:010.3e}",  -128.25), "-001.282e2");
        assert_eq!(format!("{:+010.3E}",  128.25), "+001.282E2");
        assert_eq!(format!("{:+010.3E}", -128.25), "-001.282E2");
    }

    #[test]
    fn format_float_zero() {
        assert_eq!((0.0).to_string(), "0");
        assert_eq!((-0.0).to_string(), "-0");

        assert_eq!(format!("{}",     0.0), "0");
        assert_eq!(format!("{}",    -0.0), "-0");
        assert_eq!(format!("{:?}",   0.0), "0.0");
        assert_eq!(format!("{:?}",  -0.0), "-0.0");
        assert_eq!(format!("{:+}",   0.0), "+0");
        assert_eq!(format!("{:+}",  -0.0), "-0");
        assert_eq!(format!("{:+?}",  0.0), "+0.0");
        assert_eq!(format!("{:+?}", -0.0), "-0.0");

        assert_eq!(format!("{:e}",   0.0), "0e0");
        assert_eq!(format!("{:e}",  -0.0), "-0e0");
        assert_eq!(format!("{:+e}",  0.0), "+0e0");
        assert_eq!(format!("{:+e}", -0.0), "-0e0");
        assert_eq!(format!("{:E}",   0.0), "0E0");
        assert_eq!(format!("{:E}",  -0.0), "-0E0");
        assert_eq!(format!("{:+E}",  0.0), "+0E0");
        assert_eq!(format!("{:+E}", -0.0), "-0E0");
    }

    #[test]
    fn format_float_nonfinite() {
        let nan = f32::NAN;
        assert!((-nan).is_sign_negative(), "Test setup failure: no negative NaN");

        assert_eq!(nan.to_string(), "NaN");
        // Sign of NaN is lost
        assert_eq!((-nan).to_string(), "NaN");

        assert_eq!(format!("{}",     nan), "NaN");
        assert_eq!(format!("{}",    -nan), "NaN");
        assert_eq!(format!("{:?}",   nan), "NaN");
        assert_eq!(format!("{:?}",  -nan), "NaN");
        assert_eq!(format!("{:+}",   nan), "NaN");
        assert_eq!(format!("{:+}",  -nan), "NaN");
        assert_eq!(format!("{:+?}",  nan), "NaN");
        assert_eq!(format!("{:+?}", -nan), "NaN");

        assert_eq!(format!("{:e}",   nan), "NaN");
        assert_eq!(format!("{:e}",  -nan), "NaN");
        assert_eq!(format!("{:+e}",  nan), "NaN");
        assert_eq!(format!("{:+e}", -nan), "NaN");
        assert_eq!(format!("{:E}",   nan), "NaN");
        assert_eq!(format!("{:E}",  -nan), "NaN");
        assert_eq!(format!("{:+E}",  nan), "NaN");
        assert_eq!(format!("{:+E}", -nan), "NaN");
    }

    #[test]
    fn format_float_limits() {
        // Denormal min +ve & -ve
        assert_eq!(format!("{}",    1e-45), "0.000000000000000000000000000000000000000000001");
        assert_eq!(format!("{}",   -1e-45), "-0.000000000000000000000000000000000000000000001");
        assert_eq!(format!("{:?}",  1e-45), "1e-45");
        assert_eq!(format!("{:?}", -1e-45), "-1e-45");
        assert_eq!(format!("{:e}",  1e-45), "1e-45");
        assert_eq!(format!("{:e}", -1e-45), "-1e-45");

        // Normal min +ve & -ve
        assert_eq!(format!("{}",    f32::MIN_POSITIVE), "0.000000000000000000000000000000000000011754944");
        assert_eq!(format!("{}",   -f32::MIN_POSITIVE), "-0.000000000000000000000000000000000000011754944");
        assert_eq!(format!("{:?}",  f32::MIN_POSITIVE), "1.1754944e-38");
        assert_eq!(format!("{:?}", -f32::MIN_POSITIVE), "-1.1754944e-38");
        assert_eq!(format!("{:e}",  f32::MIN_POSITIVE), "1.1754944e-38");
        assert_eq!(format!("{:e}", -f32::MIN_POSITIVE), "-1.1754944e-38");

        // Max +ve & -ve
        assert_eq!(format!("{}",   f32::MAX), "340282350000000000000000000000000000000");
        assert_eq!(format!("{}",   f32::MIN), "-340282350000000000000000000000000000000");
        assert_eq!(format!("{:?}", f32::MAX), "3.4028235e38");
        assert_eq!(format!("{:?}", f32::MIN), "-3.4028235e38");
        assert_eq!(format!("{:e}", f32::MAX), "3.4028235e38");
        assert_eq!(format!("{:e}", f32::MIN), "-3.4028235e38");
    }
}
