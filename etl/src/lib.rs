mod scrabble;

use scrabble::Rules;

pub fn transform<'a, T, U>(specification: &'a T) -> U
where
    Rules: From<&'a T>,
    U: From<Rules>,
{
    Rules::from(specification).into()
}
