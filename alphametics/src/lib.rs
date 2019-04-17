use permutator::copy::KPermutationCellIter;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const DIGITS: [u8; 10] = [0, 1, 9, 2, 7, 4, 5, 8, 6, 3];

struct Component {
    is_leading: bool,
    symbol: char,
    weight: i32,
}

pub fn solve(equation: &str) -> Option<HashMap<char, u8>> {
    let components = parse(equation);
    let k = components.len();

    let mut permutation = vec![u8::default(); k];
    let permutation = Rc::new(RefCell::new(permutation.as_mut_slice()));
    KPermutationCellIter::new(&DIGITS, k, Rc::clone(&permutation)).find_map(|_| {
        let coefficients = &**permutation.borrow();

        if is_solution(&components, coefficients) {
            Some(coefficient_by_symbol(&components, coefficients))
        } else {
            None
        }
    })
}

fn coefficient_by_symbol(components: &[Component], coefficients: &[u8]) -> HashMap<char, u8> {
    components
        .iter()
        .map(|c| c.symbol)
        .zip(coefficients.iter().cloned())
        .collect()
}

fn is_solution(components: &[Component], coefficients: &[u8]) -> bool {
    components
        .iter()
        .zip(coefficients.iter())
        .try_fold(0, |sum, (component, &coefficient)| {
            if component.is_leading && coefficient == 0 {
                None
            } else {
                Some(sum + i64::from(coefficient) * i64::from(component.weight))
            }
        })
        == Some(0)
}

fn parse(equation: &str) -> Vec<Component> {
    equation
        .splitn(2, " == ")
        .zip([1, -1].iter())
        .flat_map(|(expression, sign)| {
            expression.split(" + ").flat_map(move |term| {
                let leading_position = term.len() - 1;

                term.chars()
                    .rev()
                    .enumerate()
                    .map(move |(position, symbol)| Component {
                        is_leading: position == leading_position,
                        symbol,
                        weight: 10i32.pow(position as u32) * sign,
                    })
            })
        })
        .fold(
            HashMap::<char, Component>::new(),
            |mut components, component| {
                components
                    .entry(component.symbol)
                    .and_modify(|other| {
                        other.is_leading = other.is_leading || component.is_leading;
                        other.weight += component.weight;
                    })
                    .or_insert(component);

                components
            },
        )
        // Pending rust-lang/rust#55214
        .into_iter().map(|(_, v)| v)
        .collect()
}
