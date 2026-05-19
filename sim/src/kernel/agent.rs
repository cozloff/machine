#[derive(Debug)]
pub struct AgentBoundary {
    pub identity: &'static str,
    pub name: &'static str,
    pub origin_transform: &'static str,
    pub objective: SurvivalObjective,
}

#[derive(Debug)]
pub struct SurvivalObjective {
    pub persistence_time: &'static str,
    pub control_surface: &'static str,
    pub optionality: &'static str,
}

impl AgentBoundary {
    pub fn first_contributor() -> Self {
        Self {
            identity: "agent:1",
            name: "git_contributor",
            origin_transform: "commit_first_transform",
            objective: SurvivalObjective {
                persistence_time: "live_forever",
                control_surface: "make_money_and_power",
                optionality: "cash + ownership + leverage",
            },
        }
    }

    pub fn kernel_line(&self) -> String {
        format!(
            "{} commits the first transform: survive longer, own more, control more",
            self.identity
        )
    }
}
