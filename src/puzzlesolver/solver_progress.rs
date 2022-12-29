use std::time;

use thousands::Separable;

use super::solver::Puzzle;

pub(super) struct SolverProgress<'state> {
    state: &'state mut SolverProgressState,
}

pub(super) struct SolverProgressState {
    iterations: u64,
    pruned: u64,
    last_iterations: u64,
    last_printed: time::Instant,
    level_counts: Vec<u64>,
}

impl SolverProgressState {
    pub fn new() -> Self {
        Self {
            iterations: 0,
            pruned: 0,
            last_iterations: 0,
            last_printed: time::Instant::now(),
            level_counts: vec![],
        }
    }
}

impl<'state> SolverProgress<'state> {
    pub fn new(state: &'state mut SolverProgressState) -> Self {
        Self { state }
    }

    pub fn count(&self) -> u64 {
        self.state.iterations
    }

    pub fn incr(&mut self, puzzle: &mut impl Puzzle) {
        self.state.iterations += 1;
        *self.state.level_counts.last_mut().unwrap() += 1;
        self.show_if_necessary(puzzle);
    }

    pub fn incr_pruned(&mut self, puzzle: &mut impl Puzzle) {
        self.state.pruned += 1;
        self.show_if_necessary(puzzle);
    }

    fn show_if_necessary(&mut self, puzzle: &mut impl Puzzle) {
        let state = &mut self.state;
        if state.iterations % 100_000 != 0 {
            return;
        }

        let now = time::Instant::now();
        let since_last_printed = now.duration_since(state.last_printed);
        if since_last_printed > time::Duration::from_secs(1) {
            println!(
                "{} iterations so far... pruned:{} QPS:{}/s {}",
                state.iterations.separate_with_spaces(),
                state.pruned.separate_with_spaces(),
                ((state.iterations - state.last_iterations) as f64
                    / since_last_printed.as_secs_f64())
                .separate_with_spaces(),
                state
                    .level_counts
                    .iter()
                    .enumerate()
                    .map(|(i, count)| (i + 1, count))
                    .map(|(level, count)| format!("Level:{level} = {count}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            println!("{}", puzzle);
            state.last_printed = now;
            state.last_iterations = state.iterations;
        }
    }

    pub fn enter<'parent, 'child>(&'parent mut self) -> SolverProgress<'child>
    where
        'parent: 'child,
    {
        self.state.level_counts.push(0);
        SolverProgress {
            state: &mut self.state,
        }
    }
}

impl<'state> Drop for SolverProgress<'state> {
    fn drop(&mut self) {
        self.state.level_counts.pop();
    }
}
