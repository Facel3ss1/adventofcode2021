// Can I just say that the British names for delimiters make so much more sense
// than the American ones. Parentheses?? Braces?? These are names dreamt by the
// utterly deranged. "Parentheses" is such a pain to type as well.
#[derive(PartialEq)]
enum DelimiterType {
    Bracket,
    SquareBracket,
    CurlyBracket,
    AngleBracket,
}

impl DelimiterType {
    fn syntax_error_score(&self) -> u64 {
        match self {
            Self::Bracket => 3,
            Self::SquareBracket => 57,
            Self::CurlyBracket => 1197,
            Self::AngleBracket => 25137,
        }
    }

    fn autocomplete_score(&self) -> u64 {
        match self {
            Self::Bracket => 1,
            Self::SquareBracket => 2,
            Self::CurlyBracket => 3,
            Self::AngleBracket => 4,
        }
    }
}

enum Delimiter {
    Opening(DelimiterType),
    Closing(DelimiterType),
}

impl Delimiter {
    fn parse(c: char) -> Self {
        match c {
            '(' => Self::Opening(DelimiterType::Bracket),
            '[' => Self::Opening(DelimiterType::SquareBracket),
            '{' => Self::Opening(DelimiterType::CurlyBracket),
            '<' => Self::Opening(DelimiterType::AngleBracket),
            ')' => Self::Closing(DelimiterType::Bracket),
            ']' => Self::Closing(DelimiterType::SquareBracket),
            '}' => Self::Closing(DelimiterType::CurlyBracket),
            '>' => Self::Closing(DelimiterType::AngleBracket),
            _ => panic!(),
        }
    }
}

fn main() {
    let mut task1 = 0;
    let mut task2_scores = Vec::new();

    'lines: for line in include_str!("input.txt").lines() {
        // Stack that stores all of the opening delimiters we've seen so far
        let mut opening_stack: Vec<DelimiterType> = Vec::new();

        for c in line.chars() {
            match Delimiter::parse(c) {
                Delimiter::Opening(opening_type) => opening_stack.push(opening_type),
                Delimiter::Closing(closing_type) => match opening_stack.pop() {
                    Some(opening_type) => {
                        // The line is corrupted
                        if opening_type != closing_type {
                            task1 += closing_type.syntax_error_score();
                            continue 'lines;
                        }
                    }
                    // There are more closing delimiters than opening ones
                    None => panic!(),
                },
            }
        }

        // The line is incomplete
        if !opening_stack.is_empty() {
            let mut autocomplete_score = 0;

            while let Some(delim_type) = opening_stack.pop() {
                autocomplete_score *= 5;
                autocomplete_score += delim_type.autocomplete_score();
            }

            task2_scores.push(autocomplete_score);
        }
    }

    println!("Task 1: {}", task1);

    let middle_idx = task2_scores.len() / 2;
    let (_, &mut task2, _) = task2_scores.select_nth_unstable(middle_idx);

    println!("Task 2: {}", task2);
}
