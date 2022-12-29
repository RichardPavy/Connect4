use std::time;

use thousands::Separable;

use super::solver::Puzzle;

pub(super) struct SolverProgress {
    iterations: u64,
    pruned: u64,
    last_iterations: u64,
    last_printed: time::Instant,
}

impl SolverProgress {
    pub fn new() -> Self {
        Self {
            iterations: 0,
            pruned: 0,
            last_iterations: 0,
            last_printed: time::Instant::now(),
        }
    }

    pub fn count(&self) -> u64 {
        self.iterations
    }

    pub fn incr(&mut self, puzzle: &mut impl Puzzle) {
        self.iterations += 1;
        self.show_if_necessary(puzzle);
    }

    pub fn incr_pruned(&mut self, puzzle: &mut impl Puzzle) {
        self.pruned += 1;
        self.show_if_necessary(puzzle);
    }

    fn show_if_necessary(&mut self, puzzle: &mut impl Puzzle) {
        if self.iterations % 100_000 == 0 {
            let now = time::Instant::now();
            let since_last_printed = now.duration_since(self.last_printed);
            if since_last_printed > time::Duration::from_secs(1) {
                println!(
                    "{} iterations so far... pruned:{} QPS:{}/s",
                    self.iterations.separate_with_spaces(),
                    self.pruned.separate_with_spaces(),
                    ((self.iterations - self.last_iterations) as f64
                        / since_last_printed.as_secs_f64())
                    .separate_with_spaces()
                );
                println!("{}", puzzle);
                self.last_printed = now;
                self.last_iterations = self.iterations;
            }
        }
    }
}
