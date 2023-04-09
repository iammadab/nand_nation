#[macro_export]
macro_rules! bitstring {
    ($a:expr) => {
        Bit16::from(String::from($a))
    };
}

#[macro_export]
macro_rules! mux16test {
    ($a:expr, $b:expr, $sel:expr, $out:expr) => {
        assert_eq!(
            mux16(bitstring!($a), bitstring!($b), $sel),
            bitstring!($out)
        )
    };
}

#[macro_export]
macro_rules! and16test {
    ($a:expr, $b:expr, $out:expr) => {
        assert_eq!(and16(bitstring!($a), bitstring!($b)), bitstring!($out))
    };
}

#[macro_export]
macro_rules! or16test {
    ($a:expr, $b:expr, $out:expr) => {
        assert_eq!(or16(bitstring!($a), bitstring!($b)), bitstring!($out))
    };
}

#[macro_export]
macro_rules! not16test {
    ($a:expr, $out:expr) => {
        assert_eq!(not16(bitstring!($a)), bitstring!($out));
    };
}
