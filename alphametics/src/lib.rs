#![feature(generators, generator_trait)]

use std::collections::HashMap;
use std::iter;
use std::ops::{Generator, GeneratorState::*};
use std::pin::Pin;

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

fn iter_kpermutations(k: usize) -> impl Iterator<Item = Vec<u8>> {
    // Adapted from https://docs.python.org/3/library/itertools.html#itertools.permutations
    let mut generator = move || {
        let iter_insets = |position| (1..10 - position).rev();

        let mut cycles = (0..k).map(iter_insets).collect::<Vec<_>>();
        let mut permutation = (0..10).collect::<Vec<_>>();

        yield permutation[..k].to_vec();

        'outer: loop {
            for position in (0..k).rev() {
                match cycles[position].next() {
                    None => {
                        permutation[position..].rotate_left(1);

                        cycles[position] = iter_insets(position);
                    }
                    Some(inset) => {
                        permutation.swap(position, 10 - inset);

                        yield permutation[..k].to_vec();

                        continue 'outer;
                    }
                }
            }

            break;
        }
    };

    iter::from_fn(move || match Pin::new(&mut generator).resume() {
        Complete(()) => None,
        Yielded(permutation) => Some(permutation),
    })
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
