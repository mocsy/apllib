
#[allow(unused)]
#[macro_export]
macro_rules! map_n_and_n {
    ($func:expr, $lhs:ident, $rhs:ident) => (
        {
            $lhs.into_iter().zip($rhs).map(|(i,j)| $func(i, j)).collect()
        }
    );
}

#[allow(unused)]
#[macro_export]
macro_rules! map_n_and_one {
    ($func:expr, $lhs:ident, $rhs:ident) => (
        {
            $lhs.into_iter().map(|i| $func(i, $rhs.clone() )).collect()
        }
    );
}
