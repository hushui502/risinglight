use super::*;
use crate::{binder::BoundExpr, logical_plan::LogicalFilter};

#[derive(Debug, PartialEq, Clone)]
pub struct PhysicalFilter {
    pub expr: BoundExpr,
    pub child: Box<PhysicalPlan>,
}

impl PhysicalPlaner {
    pub fn plan_filter(&self, plan: LogicalFilter) -> Result<PhysicalPlan, PhysicalPlanError> {
        Ok(PhysicalPlan::Filter(PhysicalFilter {
            expr: plan.expr,
            child: Box::new(self.plan(*plan.child)?),
        }))
    }
}