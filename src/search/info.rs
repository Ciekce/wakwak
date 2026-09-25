use crate::engine::EngineOptions;
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
        let pv_move = thread.pv_move();

        let nodes = thread.nodes.global();
        let time = shared.time_man.elapsed();
        let nps = ((nodes as f64) / (time.as_micros().max(1) as f64) * 1e6) as u64;

        println!(
            "info depth {} seldepth {} score {} time {} nodes {nodes} nps {nps} pv {}",
            pv_move.searched_depth,
            pv_move.sel_depth,
            pv_move.display_score,
            time.as_millis(),
            pv_move.pv.display(options)
        );
    }
}
