#[macro_export]
macro_rules! tern {
    ($con:expr, $t:expr, $f:expr) => {
        if $con { $t } else { $f }
    };
}