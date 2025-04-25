//! # pyprint
//! 
//! A Rust library that provides Python-like print functionality with macros.
//! 
//! ## Features
//! 
//! - Python-style printing with customizable separators and line endings
//! - Support for regular, debug, and error printing
//! - Options for file redirection and flushing
//! - Helpful macros to reduce boilerplate
//! 
//! ## Version
//! 
//! 1.0.1
//! 
//! ## Examples
//! 
//! ```
//! use pyprint::pprn;
//! use pyprint::dprn;
//! 
//! // Basic printing (like Python's print)
//! pprn!("Hello", "World");  // Prints: Hello World
//! 
//! // Customize separator and ending
//! pprn!("Hello", "World", sep=", ", end="!\n");  // Prints: Hello, World!
//! 
//! // Print with debug formatting
//! dprn!(vec![1, 2, 3]);  // Prints the vector with default formatting
//! ```

use std::io::{Write, Result, stdout};
use std::cell::Cell;

thread_local! {
    static LAST_PRINTER_RESULT: Cell<Result<()>> = Cell::new(Ok(()));
}

/// Returns the result of the last print operation.
/// 
/// This function is useful for error handling when not using the unwrapping variants
/// of the print macros.
/// 
/// # Returns
/// 
/// The `Result` from the last printing operation.
pub fn last_printer_result() -> Result<()> {
    let mut res_copy: Cell<Result<()>> = Cell::new(Ok(()));
    LAST_PRINTER_RESULT.with(|res: &Cell<Result<()>>|{
        unsafe {
            std::ptr::copy(res, &mut res_copy, std::mem::size_of::<Cell<Result<()>>>())
        }
    });
    res_copy.into_inner()
}

/// The main printer struct used by the printing macros.
/// 
/// This struct manages the elements to print, formatting options,
/// and the output destination.
pub struct Printer {
    elements: Vec<String>,
    sep: String,
    end: String,
    file: Box<dyn Write>,
    fls: bool
}

impl Printer {
    /// Creates a new Printer with default settings.
    /// 
    /// Default settings:
    /// - separator: space (" ")
    /// - end: newline ("\n")
    /// - output: stdout
    /// - flush: false
    pub fn new() -> Self {
        Self {
            elements:Vec::new(), 
            sep: " ".to_string(), 
            end: "\n".to_string(), 
            file: Box::new(stdout()),
            fls: false
        }
    }
    
    /// Adds a string element to be printed.
    pub fn add_element(&mut self, element: String) -> &mut Self {
        self.elements.push(element);
        self
    }
    
    /// Sets the end string that is printed after all elements.
    /// 
    /// # Example
    /// 
    /// ```
    /// use pyprint::pprn;
    /// pprn!("Hello", "World", end="!");  // Prints: Hello World!
    /// ```
    pub fn set_end(&mut self, end: impl ToString) -> &mut Self {
        self.end = end.to_string();
        self
    }
    
    /// Sets the separator string used between elements.
    /// 
    /// # Example
    /// 
    /// ```
    /// use pyprint::pprn;
    /// pprn!("Hello", "World", sep=", ");  // Prints: Hello, World
    /// ```
    pub fn set_sep(&mut self, sep: impl ToString) -> &mut Self {
        self.sep = sep.to_string();
        self
    }
    
    /// Sets the output destination for printing.
    /// 
    /// # Example
    /// 
    /// ```
    /// use pyprint::pprint;
    /// use std::fs::File;
    /// 
    /// let file = File::create("output.txt").unwrap();
    /// pprint!(file=file, "Hello", "World");  // Writes to output.txt
    /// ```
    pub fn set_file(&mut self, file: impl Write + 'static) -> &mut Self {
        self.file = Box::new(file);
        self
    }

    /// Executes the print operation.
    /// 
    /// This method prints all the elements with the specified separator,
    /// followed by the end string.
    /// 
    /// # Returns
    /// 
    /// A Result that indicates whether the print operation succeeded.
    pub fn print(&mut self) -> Result<()>{
        let mut eitr = self.elements.iter();
        let opt_first = eitr.next();
        let first = match opt_first {
            Some(x) => x,
            None => {write!(self.file, "{}", self.end)?;return Ok(());}
        };
        write!(self.file, "{}", first)?;
        for s in eitr {
            write!(self.file, "{}{}", self.sep, s)?;
        }
        write!(self.file, "{}", self.end)?;
        if self.fls {
            self.file.flush()?;
        }
        Ok(())
    }
    
    /// Sets whether output should be flushed immediately.
    /// 
    /// # Example
    /// 
    /// ```
    /// use pyprint::pprn;
    /// pprn!("Progress: ", flush=true);  // Prints and flushes immediately
    /// ```
    pub fn set_flush(&mut self, fls: bool) -> &mut Self {
        self.fls = fls;
        self
    }
}

// Internal macro implementation details
#[macro_export]
macro_rules! match_variants {
    (@process [$fmt:expr, $($processed:tt)*] []) => {
        $($processed)*.print()
    };

    (@process [$fmt:expr, $($processed:tt)*] [sep=$e:expr, $($rest:tt)*]) => {
        $crate::match_variants!(@process [$fmt, $($processed)*.set_sep($e)] [$($rest)*])
    };

    (@process [$fmt:expr, $($processed:tt)*] [end=$e:expr, $($rest:tt)*]) => {
        $crate::match_variants!(@process [$fmt, $($processed)*.set_end($e)] [$($rest)*])
    };

    (@process [$fmt:expr, $($processed:tt)*] [file=$e:expr, $($rest:tt)*]) => {
        $crate::match_variants!(@process [$fmt, $($processed)*.set_file($e)] [$($rest)*])
    };

    (@process [$fmt:expr, $($processed:tt)*] [flush=$e:expr, $($rest:tt)*]) => {
        $crate::match_variants!(@process [$fmt, $($processed)*.set_flush($e)] [$($rest)*])
    };

    (@process [$fmt:expr, $($processed:tt)*] [$e:expr, $($rest:tt)*]) => {
        $crate::match_variants!(@process [$fmt, $($processed)*.add_element(format!($fmt,$e))] [$($rest)*])
    };

    (@process [$fmt:expr, $($processed:tt)*] [, $($rest:tt)*]) => {
        $crate::match_variants!(@process [$fmt, $($processed)*] [$($rest)*])
    };

    // Entry point
    ($fmt: expr, $($t:tt)*) => {
        $crate::match_variants!(@process [$fmt, $crate::Printer::new()] [$($t)*])
    };
}

/// Prints values with a specified format, returning a Result.
/// 
/// This macro is similar to Python's `print()` function, allowing for 
/// customization of separators, line endings, and output destination.
/// 
/// # Options
/// 
/// - `sep=VALUE`: Sets the separator between items (default: " ")
/// - `end=VALUE`: Sets the ending string (default: "\n")
/// - `file=VALUE`: Sets the output destination (default: stdout)
/// - `flush=BOOL`: Controls whether to flush output immediately
/// 
/// # Examples
/// 
/// ```
/// use pyprint::pprint;
/// 
/// // Basic printing
/// pprint!("Hello", "World");
/// 
/// // With custom separator and ending
/// pprint!("Hello", "World", sep=" - ", end="!\n");
/// 
/// // Print to a custom output
/// use std::fs::File;
/// let file = File::create("output.txt").unwrap();
/// pprint!(file=file, "Hello", "World");
/// ```
#[macro_export]
macro_rules! pprint {
    ($($t:tt)*) => {
        $crate::match_variants!("{}",$($t)*,)
    };
}

/// Similar to `pprint!`, but unwraps the Result.
/// 
/// This is a convenience macro that panics if printing fails.
/// 
/// # Examples
/// 
/// ```
/// use pyprint::pprn;
/// 
/// pprn!("Hello", "World", sep=", ");
/// pprn!(1, 2, 3, end=".\n");
/// ```
#[macro_export]
macro_rules! pprn {
    ($($t:tt)*) => {
        $crate::pprint!($($t)*).unwrap()
    };
}

/// Prints values in debug format.
/// 
/// This macro uses the `{:?}` formatter, making it suitable for
/// debugging complex data structures.
/// 
/// # Examples
/// 
/// ```
/// use pyprint::dprint;
/// 
/// let v = vec![1, 2, 3];
/// dprint!(v);  // Prints: [1, 2, 3]
/// 
/// let complex = ("tuple", {let mut m = std::collections::HashMap::new(); 
///                          m.insert("key", "value"); m});
/// dprint!(complex);  // Prints debug representation of the tuple
/// ```
#[macro_export]
macro_rules! dprint {
    ($($t:tt)*) => {
        $crate::match_variants!("{:?}", $($t)*,)
    };
}

/// Similar to `dprint!`, but unwraps the Result.
/// 
/// This is a convenience macro for debug printing that panics if printing fails.
#[macro_export]
macro_rules! dprn {
    ($($t:tt)*) => {
        $crate::dprint!($($t)*).unwrap()
    };
}

/// Prints to stderr.
/// 
/// Similar to `pprint!` but directs output to standard error.
/// 
/// # Examples
/// 
/// ```
/// use pyprint::eprint;
/// 
/// eprint!("Error:", "File not found");
/// ```
#[macro_export]
macro_rules! eprint {
    ($($t:tt)*) => {
        $crate::match_variants!("{}", file=std::io::stderr(), $($t)*,)
    };
}

/// Similar to `eprint!`, but unwraps the Result.
/// 
/// This is a convenience macro for error printing that panics if printing fails.
#[macro_export]
macro_rules! eprn {
    ($($t:tt)*) => {
        $crate::eprint!($($t)*).unwrap()
    };
}

/// Prints to stderr in debug format.
/// 
/// Combines the features of `eprint!` and `dprint!` to output debug format to stderr.
#[macro_export]
macro_rules! deprint {
    ($($t:tt)*) => {
        $crate::match_variants!("{:?}", file=std::io::stderr(), $($t)*,)
    };
}

/// Similar to `deprint!`, but unwraps the Result.
/// 
/// This is a convenience macro for debug error printing that panics if printing fails.
#[macro_export]
macro_rules! deprn {
    ($($t:tt)*) => {
        $crate::deprint!($($t)*).unwrap()
    };
}

#[test]
fn test_print() {
    pprn!(flush=true,"Hello",34,45,sep=";", end=".\n",34);
    dprn!(flush=true,"Hello",34,45,sep=";", end=".\n",34);
    eprn!("Hi!");
}
