macro_rules! c_abort {
    ($($fmt:expr),*) => {{
        let _ = writeln!(&mut ::std::io::stderr(), $($fmt),*);

        #[allow(unused_unsafe)]
        unsafe { ::libc::abort(); }
    }}
}

macro_rules! c_assert {
    ($cond:expr) => {
        if !$cond {
            c_abort!("Assertion failed: {}\nAborting...", stringify!($cond))
        }
    }
}