use std::io::{Write, Result, stdout};
use std::cell::Cell;

thread_local! {
    static LAST_PRINTER_RESULT: Cell<Result<()>> = Cell::new(Ok(()));
}

pub fn last_printer_result() -> Result<()> {
    let mut res_copy: Cell<Result<()>> = Cell::new(Ok(()));
    LAST_PRINTER_RESULT.with(|res: &Cell<Result<()>>|{
        unsafe {
            std::ptr::copy(res, &mut res_copy, std::mem::size_of::<Cell<Result<()>>>())
        }
    });
    res_copy.into_inner()
}

pub struct Printer {
    elements: Vec<String>,
    sep: String,
    end: String,
    file: Box<dyn Write>,
    fls: bool
}

impl Printer {
    pub fn new() -> Self {
        Self {
            elements:Vec::new(), 
            sep: " ".to_string(), 
            end: "\n".to_string(), 
            file: Box::new(stdout()),
            fls: false
        }
    }
    pub fn add_element(&mut self, element: String) -> &mut Self {
        self.elements.push(element);
        self
    }
    pub fn set_end(&mut self, end: impl ToString) -> &mut Self {
        self.end = end.to_string();
        self
    }
    pub fn set_sep(&mut self, sep: impl ToString) -> &mut Self {
        self.sep = sep.to_string();
        self
    }
    pub fn set_file(&mut self, file: impl Write + 'static) -> &mut Self {
        self.file = Box::new(file);
        self
    }

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
        Ok(())
    }
    pub fn set_flush(&mut self, fls: bool) -> &mut Self {
        self.fls = fls;
        self
    }
}



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

#[macro_export]
macro_rules! pprint {
    ($($t:tt)*) => {
        $crate::match_variants!("{}",$($t)*,)
    };
}

#[macro_export]
macro_rules! pprn {
    ($($t:tt)*) => {
        $crate::pprint!($($t)*).unwrap()
    };
}

#[macro_export]
macro_rules! dprint {
    ($($t:tt)*) => {
        $crate::match_variants!("{:?}", $($t)*,)
    };
}

#[macro_export]
macro_rules! dprn {
    ($($t:tt)*) => {
        $crate::dprint!($($t)*).unwrap()
    };
}

#[macro_export]
macro_rules! eprint {
    ($($t:tt)*) => {
        $crate::match_variants!("{}", file=std::io::stderr(), $($t)*,)
    };
}

#[macro_export]
macro_rules! eprn {
    ($($t:tt)*) => {
        $crate::eprint!($($t)*).unwrap()
    };
}


#[macro_export]
macro_rules! deprint {
    ($($t:tt)*) => {
        $crate::match_variants!("{:?}", file=std::io::stderr(), $($t)*,)
    };
}

#[macro_export]
macro_rules! deprn {
    ($($t:tt)*) => {
        $crate::edprint!($($t)*).unwrap()
    };
}



#[test]
fn test_print() {
    pprn!(flush=true,"Hello",34,45,sep=";", end=".\n",34);
    dprn!(flush=true,"Hello",34,45,sep=";", end=".\n",34);
    eprn!("Hi!");
}


