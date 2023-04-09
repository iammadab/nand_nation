#[macro_export]
macro_rules! bit {
    ($a:expr) => (Bit16::from(String::from($a)))
}

#[macro_export]
macro_rules! mux16test {
    ($a:expr, $b:expr, $sel:expr, $out:expr) => {
        assert_eq!(
            mux16(
                Bit16::from(String::from($a)),
                Bit16::from(String::from($b)),
                $sel
            ),
            Bit16::from(String::from($out))
        )
    };
}

#[macro_export]
macro_rules! and16test {
    ($a:expr, $b:expr, $out:expr) => {
        assert_eq!(
            and16(Bit16::from(String::from($a)), Bit16::from(String::from($b))),
            Bit16::from(String::from($out))
        )
    };
}

#[macro_export]
macro_rules! or16test {
    ($a:expr, $b:expr, $out:expr) => {
        assert_eq!(
            or16(Bit16::from(String::from($a)), Bit16::from(String::from($b))),
            Bit16::from(String::from($out))
        )
    };
}

#[macro_export]
macro_rules! not16test {
    ($a:expr, $out:expr) => {
        assert_eq!(
            not16(Bit16::from(String::from($a))),
            Bit16::from(String::from($out))
        );
    };
}
