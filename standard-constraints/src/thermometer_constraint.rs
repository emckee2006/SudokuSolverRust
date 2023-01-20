//! Contains the [`ThermometerConstraint`] struct for representing a thermometer constraint.

use sudoku_solver_lib::prelude::*;

/// A [`Constraint`] implementation for representing a thermometer constraint.
#[derive(Debug)]
pub struct ThermometerConstraint {
    specific_name: String,
    cells: Vec<CellIndex>,
}

impl ThermometerConstraint {
    pub fn new(specific_name: &str, cells: Vec<CellIndex>) -> Self {
        Self { specific_name: specific_name.to_owned(), cells }
    }
}

impl Constraint for ThermometerConstraint {
    fn name(&self) -> &str {
        self.specific_name.as_str()
    }

    fn init_board(&mut self, board: &mut Board) -> LogicalStepResult {
        if self.cells.is_empty() {
            return LogicalStepResult::None;
        }

        let first = self.cells[0];
        let last = self.cells[self.cells.len() - 1];
        let first_mask = board.cell(first);
        let last_mask = board.cell(last);
        let min_val = first_mask.min();
        let max_val = last_mask.max();
        let mut clear_mask = ValueMask::from_lower(min_val) | ValueMask::from_higher(max_val, board.size());
        let all_values_mask = ValueMask::from_all_values(board.size());
        let mut changed = false;

        for cell in self.cells.iter() {
            let cur_mask = board.cell(*cell);
            let new_mask = cur_mask & !(all_values_mask & clear_mask);
            if cur_mask != new_mask {
                let res = board.keep_mask(*cell, new_mask);
                if !res {
                    return LogicalStepResult::Invalid(None);
                }

                changed = true;

                clear_mask = ValueMask::from((clear_mask.raw() << 1) | 1u32);
            }
        }

        if changed {
            LogicalStepResult::Changed(None)
        } else {
            LogicalStepResult::None
        }
    }

    fn enforce(&self, board: &sudoku_solver_lib::board::Board, cell: CellIndex, val: usize) -> LogicalStepResult {
        if self.cells.is_empty() {
            return LogicalStepResult::None;
        }

        if !self.cells.contains(&cell) {
            return LogicalStepResult::None;
        }

        let mut min_value: usize = 0;
        let mut valid;
        for test_cell in self.cells.iter() {
            let mut vals = board.cell(*test_cell).to_vec();
            vals.sort();
            valid = false;
            if cell == *test_cell {
                if val > min_value {
                    min_value = val;
                } else {
                    return LogicalStepResult::Invalid(None);
                }
            } else {
                for val in vals {
                    if val > min_value {
                        min_value = val;
                        valid = true;
                        break;
                    }
                }

                if !valid {
                    return LogicalStepResult::Invalid(None);
                }
            }
        }
        LogicalStepResult::None
    }

    fn step_logic(&self, board: &mut Board, _is_brute_forcing: bool) -> LogicalStepResult {
        if self.cells.is_empty() {
            return LogicalStepResult::None;
        }

        //let elims: Vec<usize> = Vec::new();
        let mut changed;
        //let had_change = false;

        loop {
            changed = false;
            for ti in 0..board.size() {
                let cur_cell = self.cells[ti];
                let next_cell = self.cells[ti + 1];
                let cur_mask = board.cell(cur_cell);
                let next_mask = board.cell(next_cell);
                let clear_next_val_start = 

                if cur_mask.is_single() {
                    cur_mask.value()
                } else {
                    cur_mask.min()
                };
                let clear_mask = next_mask & ValueMask::from_lower_equal(clear_next_val_start);
                let res = board.clear_mask(next_cell, clear_mask);
                if !res {
                    let desc: Option<LogicalStepDesc> =
                        Some(format!("{} has no more valid candidates.", next_cell).into());
                    return LogicalStepResult::Invalid(desc);
                }
                let new_mask = board.cell(next_cell);
                if next_mask != new_mask {
                    changed = true;
                }

                let clear_cur_val_start =
                if next_mask.is_single() {
                    next_mask.value()
                } else {
                    next_mask.max()
                };

                for clear_val in clear_cur_val_start..board.size() {
                    if !board.clear_value(cur_cell, clear_val) {
                        let desc: Option<LogicalStepDesc> =
                            Some(format!("{} has no more valid candidates.", cur_cell).into());
                        return LogicalStepResult::Invalid(desc);
                    }
                }

                if cur_mask != board.cell(cur_cell) {
                    changed = true;
                }
            }

            if !changed {
                break;
            }
        }

        if changed {
            return LogicalStepResult::Changed(None);

        }

        LogicalStepResult::None

        /*
               List<int> elims = null;
               bool hadChange = false;
               bool changed;
               do
               {
                   changed = false;
                   for (int ti = 0; ti < cells.Count - 1; ti++)
                   {
                       var curCell = cells[ti];
                       var nextCell = cells[ti + 1];
                       uint curMask = board[curCell.Item1, curCell.Item2];
                       uint nextMask = board[nextCell.Item1, nextCell.Item2];
                       bool curValueSet = IsValueSet(curMask);
                       bool nextValueSet = IsValueSet(nextMask);

                       int clearNextValStart = curValueSet ? GetValue(curMask) : MinValue(curMask);
                       uint clearMask = board[nextCell.Item1, nextCell.Item2] & MaskValAndLower(clearNextValStart);
                       LogicResult clearResult = sudokuSolver.ClearMask(nextCell.Item1, nextCell.Item2, clearMask);
                       if (clearResult == LogicResult.Invalid)
                       {
                           logicalStepDescription?.Append($"{CellName(nextCell)} has no more valid candidates.");
                           return LogicResult.Invalid;
                       }
                       if (clearResult == LogicResult.Changed)
                       {
                           if (logicalStepDescription != null)
                           {
                               elims ??= new();
                               for (int v = 1; v <= MAX_VALUE; v++)
                               {
                                   if (HasValue(clearMask, v))
                                   {
                                       elims.Add(CandidateIndex(nextCell, v));
                                   }
                               }
                           }
                           changed = true;
                           hadChange = true;
                       }

                       int clearCurValStart = nextValueSet ? GetValue(nextMask) : MaxValue(nextMask);
                       clearMask = board[curCell.Item1, curCell.Item2] & MaskValAndHigher(clearCurValStart);
                       clearResult = sudokuSolver.ClearMask(curCell.Item1, curCell.Item2, clearMask);
                       if (clearResult == LogicResult.Invalid)
                       {
                           return LogicResult.Invalid;
                       }
                       if (clearResult == LogicResult.Changed)
                       {
                           if (logicalStepDescription != null)
                           {
                               elims ??= new();
                               for (int v = 1; v <= MAX_VALUE; v++)
                               {
                                   if (HasValue(clearMask, v))
                                   {
                                       elims.Add(CandidateIndex(curCell, v));
                                   }
                               }
                           }
                           changed = true;
                           hadChange = true;
                       }
                   }
               } while (changed);

               if (logicalStepDescription != null && elims != null && elims.Count > 0)
               {
                   logicalStepDescription.Append($"Re-evaluated => {sudokuSolver.DescribeElims(elims)}");
               }

               return hadChange ? LogicResult.Changed : LogicResult.None;
        */

    }
}
