
macro_rules! char_pat {
    (IDENT) => {
        'a'..='z' | 'A'..='Z' | '_'
    };
}

macro_rules! expect_token {
    ($pat:pat in ITER $iter:expr) => {
        let $pat =  $iter.next().unwrap() else {panic!()};
    };
}

pub(super) use {
    char_pat,
    expect_token,
};