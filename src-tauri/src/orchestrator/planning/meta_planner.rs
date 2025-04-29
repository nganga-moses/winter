use crate::agents::memory::planner_memory::PlannerMemoryEntry;
use crate::agents::orchestrator::protocol::PlanningStrategy;

pub trait MetaPlanner{
    fn recommend_strategy(&self, goal_id: &str, history: &[PlannerMemoryEntry])->PlanningStrategy;
}

pub struct HeuristicMetaPlanner;

impl MetaPlanner for HeuristicMetaPlanner {
    fn recommend_strategy(&self, goal_id: &str, history: &[PlannerMemoryEntry]) -> PlanningStrategy {
        if let Some(last) = history.last(){
            if last.score.unwrap_or(0) >= 7 {
                return PlanningStrategy::ReusePlan{
                    plan_id: last.plan_id.clone(),
                };
            } else if last.revision_id.unwrap_or(0) < 3 {
                return PlanningStrategy::ReviseLast{
                    plan_id: last.plan_id.clone(),
                };
            }
        }

        PlanningStrategy::GenerateFresh
    }
}