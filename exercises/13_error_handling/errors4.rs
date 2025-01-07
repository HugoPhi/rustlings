#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.

        // if value < 0 {
        //     Err(CreationError::Negative)
        // } else if value == 0 {
        //     Err(CreationError::Zero)
        // } else {
        //     Ok(Self(value as u64))
        // }
        // warning: `if` chain can be rewritten with `match`
        //   --> exercises/13_error_handling/errors4.rs:13:9
        //         |
        //      13 | /         if value < 0 {
        //      14 | |             Err(CreationError::Negative)
        //      15 | |         } else if value == 0 {
        //      16 | |             Err(CreationError::Zero)
        //      17 | |         } else {
        //      18 | |             Ok(Self(value as u64))
        //      19 | |         }
        //         | |_________^
        //         |
        //         = help: consider rewriting the `if` chain to use `cmp` and `match`
        //         // = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#comparison_chain
        //         = note: `#[warn(clippy::comparison_chain)]` on by default

        match value.cmp(&0) {
            std::cmp::Ordering::Less => Err(CreationError::Negative),
            std::cmp::Ordering::Equal => Err(CreationError::Zero),
            std::cmp::Ordering::Greater => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
