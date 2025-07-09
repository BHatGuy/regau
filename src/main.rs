#[derive(Debug, PartialEq, Eq)]
enum Atom {
    Start,
    End,
    Any,
    Literal(char),
}

fn parse(regex: &str) -> Vec<Atom> {
    let mut atoms = Vec::new();

    for c in regex.chars() {
        match c {
            '^' => atoms.push(Atom::Start),
            '$' => atoms.push(Atom::End),
            '.' => atoms.push(Atom::Any),
            _ => atoms.push(Atom::Literal(c)),
        }
    }

    atoms
}

struct Match<'a> {
    start: usize,
    end: usize,
    open_atoms: &'a [Atom],
}

fn rmatch(input: &str, regex: &[Atom]) -> bool {
    let mut possible_matches = Vec::new();

    for (i, chr) in input.chars().enumerate() {
        possible_matches.push(Match {
            start: i,
            end: i,
            open_atoms: regex,
        });
        let mut next_matches = Vec::new();

        for mut mat in possible_matches {
            let matches = match mat.open_atoms[0] {
                Atom::Start => mat.start == 0,
                Atom::End => mat.end == input.len(),
                Atom::Any => true,
                Atom::Literal(l) => l == chr,
            };
            if matches {
                mat.end += 1;
                mat.open_atoms = &mat.open_atoms[1..];
                if mat.open_atoms.is_empty() {
                    return true;
                }
                next_matches.push(mat);
            }
        }

        possible_matches = next_matches;
    }

    false
}

fn main() {
    let test_regex = parse("....");
    let res = rmatch("abc", &test_regex);
    println!("{res}");
}

#[cfg(test)]
mod match_tests {
    use super::*;

    #[test]
    fn match_literals() {
        let regex = parse("abcd");

        assert!(rmatch("abcd", &regex));
        assert!(rmatch("_abcd_", &regex));

        assert!(!rmatch("_abc_", &regex));
        assert!(!rmatch("abc", &regex));
    }

    #[test]
    fn match_any() {
        let regex = parse("a.c.");

        assert!(rmatch("abcd", &regex));
        assert!(rmatch("aacc", &regex));
        assert!(rmatch("_aacc_", &regex));

        assert!(!rmatch("abc", &regex));
        assert!(!rmatch("_abc", &regex));
    }
}
