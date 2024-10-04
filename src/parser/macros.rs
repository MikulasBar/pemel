
macro_rules! char_pat {
    (IDENT) => {
        'a'..='z' | 'A'..='Z' | '_'
    };
}

macro_rules! expect_token {
    ($pat:pat in ITER $iter:expr) => {
        #[allow(irrefutable_let_patterns)]
        let $pat =  $iter.next().unwrap() else {
            panic!("Unexpected token {:?}", $iter.next().unwrap())
        };
    };
}

pub(super) use {
    char_pat,
    expect_token,
};