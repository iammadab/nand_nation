#[macro_export]
macro_rules! bit16string {
    ($a:expr) => {
        Bit16::from(String::from($a))
    };
}

#[macro_export]
macro_rules! bit8string {
    ($a:expr) => {
        Bit8::from(String::from($a))
    };
}

#[macro_export]
macro_rules! mux16test {
    ($a:expr, $b:expr, $sel:expr, $out:expr) => {
        assert_eq!(
            mux16(bit16string!($a), bit16string!($b), $sel),
            bit16string!($out)
        )
    };
}

#[macro_export]
macro_rules! and16test {
    ($a:expr, $b:expr, $out:expr) => {
        assert_eq!(
            and16(bit16string!($a), bit16string!($b)),
            bit16string!($out)
        )
    };
}

#[macro_export]
macro_rules! or16test {
    ($a:expr, $b:expr, $out:expr) => {
        assert_eq!(or16(bit16string!($a), bit16string!($b)), bit16string!($out))
    };
}

#[macro_export]
macro_rules! not16test {
    ($a:expr, $out:expr) => {
        assert_eq!(not16(bit16string!($a)), bit16string!($out));
    };
}
