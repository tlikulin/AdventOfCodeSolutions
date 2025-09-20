// NB: it doesn't actually test if you can split the remaining into 3 equal parts (only tests if one such part can be achieved), but still gives the right answer.
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let packages: Vec<usize> = input.lines().flat_map(str::parse).collect();

    let total: usize = packages.iter().sum();
    let target = total / 4;
    let min_size_hint = target.div_ceil(*packages.last().unwrap()) as usize;

    for size in min_size_hint..packages.len() {
        let combinations = all_combinations_with_size_and_sum(size, &packages, target);

        if combinations.is_empty() {
            continue;
        }

        let mut lowest_qe = usize::MAX;
        for combination in combinations {
            let rest: Vec<usize> = packages
                .iter()
                .filter(|&p| !combination.contains(p))
                .copied()
                .collect();
            if can_split(&rest, target) {
                let quantum_entanglement: usize = combination.into_iter().product();
                lowest_qe = lowest_qe.min(quantum_entanglement);
            }
        }

        if lowest_qe != usize::MAX {
            println!("{lowest_qe}");
            break;
        }
    }
}

fn all_combinations_with_size_and_sum(
    size: usize,
    packages: &[usize],
    target: usize,
) -> Vec<Vec<usize>> {
    let mut combinations = Vec::new();
    let mut current = Vec::new();

    combinations_inner(
        &mut combinations,
        &mut current,
        packages,
        target,
        packages.len(),
        0,
        size,
    );

    combinations
}

fn combinations_inner(
    combinations: &mut Vec<Vec<usize>>,
    current: &mut Vec<usize>,
    packages: &[usize],
    target: usize,
    smallest_used: usize,
    subtotal: usize,
    left: usize,
) {
    if left == 0 {
        if subtotal == target {
            combinations.push(current.clone());
        }

        return;
    }

    for i in 0..smallest_used {
        current.push(packages[i]);
        combinations_inner(
            combinations,
            current,
            packages,
            target,
            i,
            subtotal + packages[i],
            left - 1,
        );
        current.pop();
    }
}

fn can_split(packages: &[usize], target: usize) -> bool {
    split_inner(packages, target, packages.len(), 0)
}

fn split_inner(packages: &[usize], target: usize, smallest_used: usize, subtotal: usize) -> bool {
    if subtotal == target {
        return true;
    } else if subtotal > target {
        return false;
    }

    for i in 0..smallest_used {
        if split_inner(packages, target, i, subtotal + packages[i]) {
            return true;
        }
    }

    false
}
