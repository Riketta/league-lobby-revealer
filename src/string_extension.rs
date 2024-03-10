pub trait StringExt {
    fn split_as_arguments(self) -> Vec<String>;
}

impl StringExt for String {
    fn split_as_arguments(self) -> Vec<String> {
        let mut arguments = Vec::<String>::new();

        let mut is_escaped = false;
        let mut is_in_quotes = false;
        let mut current_argument = None;

        for char in self.chars() {
            if current_argument.is_none() {
                current_argument = Some(String::with_capacity(30));
            }

            match char {
                '\\' => {
                    if !is_escaped && !is_in_quotes {
                        is_escaped = true;
                        continue;
                    }
                }
                '"' => {
                    if !is_escaped {
                        is_in_quotes = !is_in_quotes;
                    }
                }
                ' ' => {
                    // Space after block of text in quotes - current argument done.
                    if !is_in_quotes {
                        is_escaped = false;
                        arguments.push(current_argument.take().unwrap());
                        continue;
                    }
                }
                _ => (),
            }

            if let Some(ref mut arg) = current_argument {
                arg.push(char)
            }

            if is_escaped {
                is_escaped = false;
            }
        }
        if current_argument.is_some() {
            arguments.push(current_argument.unwrap());
        }

        arguments
    }
}
