//! # Quick Input
//!
//! This lightweight library offers a quick and easy way of handling user input, which can then
//! be assigned to a variable directly. All functions handle the possibility of invalid values by
//! looping until the desired type is inputted.
//!
//! Both an initial prompt message and error message are customisable and optional.
//! If the error message is left empty (None), a default error message will be displayed
//! (relaying on the default message should be avoided when possible).
//!
//! # Author
//! - Kevin Claramonte Soler (kevclasol@proton.me).
//! - 16/03/2025

// ----- BASIC ----- //

use std::io;
use std::io::Write;

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type a string of text which will then be returned.
///
/// Provides an information message on the same line as the prompt if Some(...)
/// is provided, and just the prompt if None is provided.
///
/// # RETURNS #
/// A trimmed String value provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_string;
/// let user_str_with_msg = read_string(Some("Please input some text: "));
///
/// let user_str: String = read_string(None);
/// ```
pub fn read_string(msg: Option<&str>) -> String {
    let mut input = String::new();

    if msg.is_some() {
        print!("{}", msg.unwrap());
        flush_and_read(&mut input);
    } else {
        flush_and_read(&mut input);
    }

    input.trim().to_string()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (i32) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type i32 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_i32;
/// let user_i32_with_msg = read_i32(Some("Please input a number: "), Some("Please input a valid number."));
///
/// let user_i32: i32 = read_i32(None, None);
/// ```
pub fn read_i32(msg: Option<&str>, err_msg: Option<&str>) -> i32 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<i32>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<i32>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (32 bits).");
            }
        }
    } else {
        while input.trim().parse::<i32>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<i32>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (32 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (u32) which will then be returned.
/// If user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type u32 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_u32;
/// let user_u32_with_msg = read_u32(Some("Please input a number: "), Some("Please input a valid number."));
///
/// let user_u32: u32 = read_u32(None, None);
/// ```
pub fn read_u32(msg: Option<&str>, err_msg :Option<&str>) -> u32 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<u32>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<u32>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (32 bits).");
            }
        }
    } else {
        while input.trim().parse::<i32>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<i32>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (32 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type a real number with double precision (f64) which will then be returned.
/// Both '.' and ',' are accepted as separators for the decimal part (Ex: 12.3 and 45,67).
/// If the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// A floating point value of type f64 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_f64;
/// let user_f64_with_msg = read_f64(Some("Please input a number with decimals: "), Some("Please input a valid number."));
///
/// let user_f64: f64 = read_f64(None, None);
/// ```
pub fn read_f64(msg: Option<&str>, err_msg: Option<&str>) -> f64 {
    let mut input = String::new();

    if msg.is_some() {
        while input.replace(',', ".").trim().parse::<f64>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.replace(',', ".").trim().parse::<f64>().is_err() {
                show_error_message(err_msg, "Please enter a valid real number (64 bits).");
            }
        }
    } else {
        while input.trim().parse::<f64>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.replace(',', ".").trim().parse::<f64>().is_err() {
                show_error_message(err_msg, "Please enter a valid real number (64 bits).");
            }
        }
    }

    input.replace(',', ".").trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type a character (char) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// # RETURNS #
/// A single character (char) provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_char;
/// let user_char_with_msg = read_char(Some("Please input a character: "));
///
/// let user_char: char = read_char(None);
/// ```
pub fn read_char(msg: Option<&str>) -> char {
    let mut input = String::from(".");

    if msg.is_some() {
        input.clear();
        print!("{}", msg.unwrap());
        flush_and_read(&mut input);
    } else {
        input.clear();
        flush_and_read(&mut input);
    }

    input.trim().chars().next().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type a boolean value (bool) manually, which will then be returned.
/// This function is not case-sensitive, so values like True or fAlSe will still work.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// A boolean value (bool) provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_bool;
/// let user_bool_with_msg = read_bool(Some("Please input a boolean value: "), Some("Please input true or false."));
///
/// let user_bool: bool = read_bool(None, None);
/// ```
pub fn read_bool(msg: Option<&str>, err_msg: Option<&str>) -> bool {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<bool>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            input = input.trim().to_lowercase();

            if input.parse::<bool>().is_err() {
                show_error_message(err_msg, "Please enter a valid boolean value (true / false).");
            }
        }
    } else {
        while input.trim().parse::<bool>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            input = input.trim().to_lowercase();

            println!("{input}");

            if input.parse::<bool>().is_err() {
                show_error_message(err_msg, "Please enter a valid boolean value (true / false).");
            }
        }
    }

    input.trim().parse::<bool>().unwrap()
}

// ----- EXTRA ----- //

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type a string of text which will then be returned.
///
/// Provides an information message on the same line as the prompt if Some(...)
/// is provided, and just the prompt if None is provided.
///
/// # RETURNS #
/// A non-trimmed String value provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_string_untrimmed;
/// let user_str_with_msg = read_string_untrimmed(Some("Please input some text: "));
///
/// let user_str: String = read_string_untrimmed(None);
/// ```
pub fn read_string_untrimmed(msg: Option<&str>) -> String {
    let mut input = String::new();

    if msg.is_some() {
        print!("{}", msg.unwrap());
        flush_and_read(&mut input);
    } else {
        flush_and_read(&mut input);
    }
    input
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type a real number with single precision (f32) which will then be returned.
/// Both '.' and ',' are accepted as separators for the decimal part (Ex: 12.3 and 45,67).
/// If the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// A floating point value of type f32 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_f32;
/// let user_f32_with_msg = read_f32(Some("Please input a number with decimals: "), Some("Please input a valid number."));
///
/// let user_f32: f32 = read_f32(None, None);
/// ```
pub fn read_f32(msg: Option<&str>, err_msg: Option<&str>) -> f32 {
    let mut input = String::new();

    if msg.is_some() {
        while input.replace(',', ".").trim().parse::<f32>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.replace(',', ".").trim().parse::<f32>().is_err() {
                show_error_message(err_msg, "Please enter a valid real number (32 bits).");
            }
        }
    } else {
        while input.trim().parse::<f32>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.replace(',', ".").trim().parse::<f32>().is_err() {
                show_error_message(err_msg, "Please enter a valid real number (32 bits).");
            }
        }
    }

    input.replace(',', ".").trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (i8) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type i8 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_i8;
/// let user_i8_with_msg = read_i8(Some("Please input a number: "),Some("Please input a valid number."));
///
/// let user_i8: i8 = read_i8(None, None);
/// ```
pub fn read_i8(msg: Option<&str>, err_msg: Option<&str>) -> i8 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<i8>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<i8>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (8 bits).");
            }
        }
    } else {
        while input.trim().parse::<i8>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<i8>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (8 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (u8) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type u8 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_u8;
/// let user_u8_with_msg = read_u8(Some("Please input a number: "), Some("Please input a valid number."));
///
/// let user_u8: u8 = read_u8(None, None);
/// ```
pub fn read_u8(msg: Option<&str>, err_msg: Option<&str>) -> u8 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<u8>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<u8>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (8 bits).");
            }
        }
    } else {
        while input.trim().parse::<u8>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<u8>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (8 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (i16) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type i16 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_i16;
/// let user_i16_with_msg = read_i16(Some("Please input a number: "), Some("Please input a valid number."));
///
/// let user_i16: i16 = read_i16(None, None);
/// ```
pub fn read_i16(msg: Option<&str>, err_msg: Option<&str>) -> i16 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<i16>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<i16>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (16 bits).");
            }
        }
    } else {
        while input.trim().parse::<i16>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<i16>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (16 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (u16) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type u16 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_u16;
/// let user_u16_with_msg = read_u16(Some("Please input a number: "), Some("Please input a valid number."));
///
/// let user_u16: u16 = read_u16(None, None);
/// ```
pub fn read_u16(msg: Option<&str>, err_msg: Option<&str>) -> u16 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<u16>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<u16>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (16 bits).");
            }
        }
    } else {
        while input.trim().parse::<u16>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<u16>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (16 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (i64) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type i64 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_i64;
/// let user_i64_with_msg = read_i64(Some("Please input a number: "), Some("Please input a valid number"));
///
/// let user_i64: i64 = read_i64(None, None);
/// ```
pub fn read_i64(msg: Option<&str>, err_msg: Option<&str>) -> i64 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<i64>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<i64>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (64 bits).");
            }
        }
    } else {
        while input.trim().parse::<i64>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<i64>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (64 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (u64) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type u64 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_u64;
/// let user_u64_with_msg = read_u64(Some("Please input a number: "), Some("Please input a valid number."));
///
/// let user_u64: u64 = read_u64(None, None);
/// ```
pub fn read_u64(msg: Option<&str>, err_msg: Option<&str>) -> u64 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<u64>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<u64>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (64 bits).");
            }
        }
    } else {
        while input.trim().parse::<u64>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<u64>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (64 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (i128) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type i128 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_i128;
/// let user_i128_with_msg = read_i128(Some("Please input a number: "), Some("Please input a valid number."));
///
/// let user_i128: i128 = read_i128(None, None);
/// ```
pub fn read_i128(msg: Option<&str>, err_msg: Option<&str>) -> i128 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<i128>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<i128>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (128 bits).");
            }
        }
    } else {
        while input.trim().parse::<i128>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<i128>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (128 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (u128) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type u128 provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_u128;
/// let user_u128_with_msg = read_u128(Some("Please input a number: "), Some("Please input a valid number."));
///
/// let user_u128: u128 = read_u128(None, None);
/// ```
pub fn read_u128(msg: Option<&str>, err_msg: Option<&str>) -> u128 {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<u128>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<u128>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (128 bits).");
            }
        }
    } else {
        while input.trim().parse::<u128>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<u128>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (128 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (isize) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type isize provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_isize;
/// let user_isize_with_msg = read_isize(Some("Please input a number: "), Some("Please input a valid number"));
///
/// let user_isize: isize = read_isize(None, None);
/// ```
pub fn read_isize(msg: Option<&str>, err_msg: Option<&str>) -> isize {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<isize>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<isize>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (32/64 bits).");
            }
        }
    } else {
        while input.trim().parse::<isize>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<isize>().is_err() {
                show_error_message(err_msg, "Please enter a valid number (32/64 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}

/// # ARGUMENTS #
/// 'msg' (Option<&str>) - an optional message which will be printed at
/// the same line as the input prompt. Must be set to Some("...") or None.
///
/// 'err_msg' (Option<&str>) - an optional error message which will be printed
/// if the user inputs an invalid value. Must be set to Some("...") or None.
///
/// # DESCRIPTION #
/// Prompts the user to type an integer value (usize) which will then be returned.
/// In case the user writes an invalid value, they will be prompted to try again.
///
/// Provides an information message on the same line as the prompt if Some("...")
/// is provided, and just the prompt if None is provided.
///
/// If err_msg is set to None, a default message will be shown.
///
/// # RETURNS #
/// An integer value of type usize provided by the user.
///
/// # EXAMPLES #
/// ```
/// use quick_input::read_usize;
/// let user_usize_with_msg = read_usize(Some("Please input a number: "), Some("Please input a valid number."));
///
/// let user_usize: usize = read_usize(None, None);
/// ```
pub fn read_usize(msg: Option<&str>, err_msg: Option<&str>) -> usize {
    let mut input = String::new();

    if msg.is_some() {
        while input.trim().parse::<usize>().is_err() {
            input.clear();
            print!("{}", msg.unwrap());
            flush_and_read(&mut input);

            if input.trim().parse::<usize>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (32/64 bits).");
            }
        }
    } else {
        while input.trim().parse::<usize>().is_err() {
            input.clear();
            flush_and_read(&mut input);

            if input.trim().parse::<usize>().is_err() {
                show_error_message(err_msg, "Please enter a valid positive number (32/64 bits).");
            }
        }
    }

    input.trim().parse().unwrap()
}


// ----- PRIVATE METHODS ----- //

/// # Arguments #
/// 'input' (&mut String) - Mutable reference to the variable containing
/// an empty String, which is returned at the end of all read_* methods.
///
/// # Description #
/// Private method used to force the print!() macro to show the &str message provided
/// on the same line as the input prompt.
///
/// This function also obtains the value typed by the user and assings it
/// to the "input" variable through the mutable reference provided.
fn flush_and_read(input: &mut String) {
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(input)
        .expect("Unable to read from stdin.");
}

/// # Arguments #
/// 'err_msg' (Option<&str>) - Custom error message which will be displayed in case
/// the user provides an invalid value. Must be set to Some("...") or None.
///
/// 'def_err_msg' (&str) - Default error message that will be shown if the user provides
/// an invalid value and the provided error message (err_msg) is set to None.
///
/// # Description #
/// Private function used to display a custom error message if the users provides an invalid value.
/// This function will display a default error message if the provided custom error message is set to None.
fn show_error_message(err_msg: Option<&str>, def_err_msg: &str) {
    if err_msg.is_some() {
        println!("{}", err_msg.unwrap());
        println!("---");
    } else {
        println!("{def_err_msg}");
        println!("---");
    }
}


#[cfg(test)]
mod tests {

}
