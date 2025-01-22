macro_rules! char_pat {
    (IDENT) => {
        'a'..='z' | 'A'..='Z' | '_'
    };
}

macro_rules! expect_token {
    ($pat:pat in ITER $iter:expr) => {
        #[allow(irrefutable_let_patterns)]
        let $pat = $iter.next().unwrap() else {
            panic!("Unexpected token {:?}", $iter.next().unwrap())
        };
    };
}

// If the pattern is not matched the function returns an error
// The return statements is used twice, because it use the Let ... = (if let ... else ...) else ... pattern
macro_rules! expect_token_ret {
    ($pat:pat in ITER $iter:expr) => {
        #[allow(irrefutable_let_patterns)]
        let $pat = (if let Some(_) = $iter.peek() {
            $iter.next().unwrap()
        } else {
            // there is no token to return
            return Err(ParseError::UnexpectedToken(Token::EOF));
        }) else {
            // the token is not the expected one
            return Err(ParseError::UnexpectedToken($iter.next().unwrap()));
        };
    };
}

pub(super) use {char_pat, expect_token, expect_token_ret};
