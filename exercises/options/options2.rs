// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let word = optional_target {
            assert_eq!(word.unwrap(), target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let integer = optional_integers.pop() {
            match integer {
                Some(inte) => {
                    if inte.is_some() {
                        if let val = inte.unwrap() {
                            assert_eq!(inte.unwrap(), cursor);
                            cursor -= 1;
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }

        assert_eq!(cursor, 0);
    }
}
