use std::collections::HashMap;
use std::iter;

struct Component {
    is_leading: bool,
    symbol: char,
    weight: i32,
}

pub fn solve(equation: &str) -> Option<HashMap<char, u8>> {
    let components = parse(equation);

    iter_kpermutations(components.len()).find_map(|coefficients| {
        if is_solution(&components, &coefficients) {
            Some(coefficient_by_symbol(&components, &coefficients))
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

// Adapted from https://docs.python.org/3/library/itertools.html#itertools.permutations
pub fn iter_kpermutations(k: usize) -> impl Iterator<Item = Vec<u8>> {
    let iter_insets = |position| (1..10 - position).rev();

    let mut iters_insets = (0..k).map(iter_insets).collect::<Vec<_>>();
    let mut permutation = (0..10).collect::<Vec<_>>();

    iter::once(permutation[..k].to_owned()).chain(iter::from_fn(move || {
        (0..k)
            .rev()
            .find_map(|position| match iters_insets[position].next() {
                None => {
                    permutation[position..].rotate_left(1);
                    iters_insets[position] = iter_insets(position);

                    None
                }
                Some(inset) => {
                    permutation.swap(position, 10 - inset);

                    Some(permutation[..k].to_owned())
                }
            })
    }))
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
