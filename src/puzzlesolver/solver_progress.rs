use std::time;

use thousands::Separable;

pub(super) struct SolverProgress {
    iterations: u64,
    last_iterations: u64,
    last_printed: time::Instant,
}

impl SolverProgress {
    pub fn new() -> Self {
        Self {
            iterations: 0,
            last_iterations: 0,
            last_printed: time::Instant::now(),
        }
    }

    pub fn incr(&mut self) {
        self.iterations += 1;
        if self.iterations % 100_000 == 0 {
            let now = time::Instant::now();
            let since_last_printed = now.duration_since(self.last_printed);
            if since_last_printed > time::Duration::from_secs(1) {
                println!(
                    "{} billion iterations so far... ({}/s)",
                    self.iterations.separate_with_spaces(),
                    ((self.iterations - self.last_iterations) as f64
                        / since_last_printed.as_secs_f64())
                    .separate_with_spaces()
                );
                self.last_printed = now;
                self.last_iterations = self.iterations;
            }
        }
    }
}
