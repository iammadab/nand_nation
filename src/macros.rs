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
macro_rules! bit2string {
    ($a:expr) => {
        Bit2::from(String::from($a))
    };
}

#[macro_export]
macro_rules! bit3string {
    ($a:expr) => {
        Bit3::from(String::from($a))
    };
}

#[macro_export]
macro_rules! bit4string {
    ($a:expr) => {
        Bit4::from(String::from($a))
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

#[macro_export]
macro_rules! mux4way16test {
    ($a:expr, $b:expr, $c:expr, $d:expr, $sel:expr, $out:expr) => {
        assert_eq!(
            mux4way16(
                [
                    bit16string!($a),
                    bit16string!($b),
                    bit16string!($c),
                    bit16string!($d),
                ],
                bit2string!($sel),
            ),
            bit16string!($out)
        );
    };
}

#[macro_export]
macro_rules! mux8way16test {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $sel:expr, $out:expr) => {
        assert_eq!(
            mux8way16(
                [
                    bit16string!($a),
                    bit16string!($b),
                    bit16string!($c),
                    bit16string!($d),
                    bit16string!($e),
                    bit16string!($f),
                    bit16string!($g),
                    bit16string!($h),
                ],
                bit3string!($sel),
            ),
            bit16string!($out)
        );
    };
}

#[macro_export]
macro_rules! add16test {
    ($a:expr, $b:expr, $out:expr) => {
        assert_eq!(
            add16(bit16string!($a), bit16string!($b)),
            bit16string!($out)
        );
    };
}
