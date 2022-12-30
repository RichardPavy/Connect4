use std::time;

use thousands::Separable;

use crate::puzzlesolver::puzzle_piece::ShapeIdx;

use super::solver::Puzzle;

pub(super) struct SolverProgress<'state> {
    state: &'state mut SolverProgressState,
    shape_idx: ShapeIdx,
}

pub(super) struct SolverProgressState {
    iterations: u64,
    pruned: u64,
    last_iterations: u64,
    last_printed: time::Instant,
    shapes_status: ShapesStatus,
}

impl SolverProgressState {
    pub fn new(shapes_status: ShapesStatus) -> Self {
        Self {
            iterations: 0,
            pruned: 0,
            last_iterations: 0,
            last_printed: time::Instant::now(),
            shapes_status,
        }
    }
}

pub(super) struct ShapesStatus {
    used: Vec<bool>,
    remaining: usize,
}

impl ShapesStatus {
    pub fn of<T>(shapes: &[T]) -> Self {
        Self {
            used: shapes.iter().map(|_| false).collect(),
            remaining: shapes.len(),
        }
    }
}

impl<'state> SolverProgress<'state> {
    pub fn new(state: &'state mut SolverProgressState) -> Self {
        Self {
            state,
            shape_idx: 0,
        }
    }

    pub fn count(&self) -> u64 {
        self.state.iterations
    }

    pub fn incr(&mut self, puzzle: &mut impl Puzzle) {
        self.state.iterations += 1;
        self.show_if_necessary(puzzle);
    }

    pub fn incr_pruned(&mut self, puzzle: &mut impl Puzzle) {
        self.state.pruned += 1;
        self.show_if_necessary(puzzle);
    }

    fn show_if_necessary(&mut self, puzzle: &mut impl Puzzle) {
        const SHOW_PROGRESS_MASK: u64 = (1 << 17) - 1;
        if self.state.iterations & SHOW_PROGRESS_MASK != 0 {
            return;
        }
        let now = time::Instant::now();
        let since_last_printed = now.duration_since(self.state.last_printed);
        if since_last_printed > time::Duration::from_secs(1) {
            self.print(puzzle);
            let state = &mut self.state;
            state.last_printed = now;
            state.last_iterations = state.iterations;
        }
    }

    pub fn print(&mut self, puzzle: &mut impl Puzzle) {
        let state = &mut self.state;
        let now = time::Instant::now();
        let since_last_printed = now.duration_since(state.last_printed);
        println!(
            "{} iterations so far...    pruned:{}    QPS:{}/s",
            state.iterations.separate_with_spaces(),
            state.pruned.separate_with_spaces(),
            ((state.iterations - state.last_iterations) as f64 / since_last_printed.as_secs_f64())
                .separate_with_spaces(),
        );
        println!("{}", puzzle);
    }

    pub fn enter<'parent, 'child>(&'parent mut self, shape_idx: ShapeIdx) -> SolverProgress<'child>
    where
        'parent: 'child,
    {
        self.state.shapes_status.remaining -= 1;
        self.shapes_used_mut()[shape_idx as usize] = true;
        SolverProgress {
            state: &mut self.state,
            shape_idx,
        }
    }

    pub fn shapes_used(&self) -> &Vec<bool> {
        &self.state.shapes_status.used
    }

    pub fn shapes_used_mut(&mut self) -> &mut Vec<bool> {
        &mut self.state.shapes_status.used
    }

    pub fn finish(&self) -> bool {
        self.state.shapes_status.remaining == 0
    }
}

impl<'state> Drop for SolverProgress<'state> {
    fn drop(&mut self) {
        self.state.shapes_status.remaining += 1;
        let shape_idx = self.shape_idx;
        self.shapes_used_mut()[shape_idx as usize] = false;
    }
}
