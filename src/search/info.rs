use crate::engine::EngineOptions;
use crate::score::Score;
use crate::search::{SharedData, ThreadData};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SearchInfo {
    Full,
    Minimal,
    None,
}

impl SearchInfo {
    #[inline]
    pub fn depth(self, thread: &ThreadData, shared: &SharedData, options: EngineOptions) {
        let nodes = thread.nodes.global();
        let time = shared.time_man.elapsed();
        let nps = ((nodes as f64) / (time.as_micros().max(1) as f64) * 1e6) as u64;

        for (pv_idx, root_move) in thread.root_moves[..thread.multipv].iter().enumerate() {
            let mut score = root_move.display_score;
            let mut upper_bound = root_move.upper_bound;
            let mut lower_bound = root_move.lower_bound;

            if score == -Score::INFINITE {
                score = root_move.previous_score;
                upper_bound = false;
                lower_bound = false;
            }

            if score == -Score::INFINITE {
                break;
            }

            print!("info");

            if options.multipv > 1 {
                print!(" multipv {}", pv_idx + 1);
            }

            print!(
                " depth {} seldepth {} score {}",
                root_move.searched_depth, root_move.sel_depth, score,
            );

            if upper_bound {
                print!(" upperbound");
            }

            if lower_bound {
                print!(" lowerbound");
            }

            println!(
                " time {} nodes {nodes} nps {nps} pv {}",
                time.as_millis(),
                root_move.pv.display(options)
            );
        }
    }
}
