use crate::kernel::action::ActionTicket;
use crate::kernel::objective::RewardScore;
use crate::kernel::reality::RealitySignal;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct BusinessOrder {
    pub constraint: &'static str,
    pub priority: &'static str,
    pub command: String,
    pub forbidden: &'static str,
    pub success_signal: &'static str,
    pub order_path: &'static str,
}

impl BusinessOrder {
    pub fn from_state(ticket: &ActionTicket, signal: &RealitySignal, score: &RewardScore) -> Self {
        if signal.is_open_loop() {
            if !ticket.payment_ready || !ticket.outreach_ready {
                return Self {
                    constraint: "missing_integrations",
                    priority: "make_money_reachable_before_outreach",
                    command: missing_integration_command(ticket),
                    forbidden: "do_not_send_quote_until_payment_and_outreach_are_configured",
                    success_signal: "payment_configured|outreach_configured",
                    order_path: "out/business_order.md",
                };
            }

            return Self {
                constraint: "cash_and_reality_signal",
                priority: "collect_deposit_or_disqualification",
                command: format!(
                    "Create or verify payment link, then send {} and {}. Require ${:.2} deposit or explicit counter within {} hours.",
                    ticket.message_path,
                    ticket.payment_request_path,
                    ticket.required_deposit,
                    ticket.response_deadline_hours
                ),
                forbidden: "do_not_rerun_or_refine_until_customer_response_exists",
                success_signal: "accepted|countered|paid|rejected|ignored",
                order_path: "out/business_order.md",
            };
        }

        let priority = if score.total_reward >= 0.0 {
            "compound_winning_signal"
        } else {
            "correct_model_before_next_quote"
        };

        Self {
            constraint: "prediction_error",
            priority,
            command: String::from(
                "Update model assumptions from observed error before sending another quote.",
            ),
            forbidden: "do_not_ignore_reality_signal",
            success_signal: "updated_priors|changed_policy|next_quote",
            order_path: "out/business_order.md",
        }
    }

    pub fn write(&self) -> io::Result<()> {
        if let Some(parent) = Path::new(self.order_path).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(self.order_path, self.to_markdown())
    }

    fn to_markdown(&self) -> String {
        format!(
            r#"# Business Order

Constraint: {constraint}
Priority: {priority}

Command:

{command}

Forbidden:

{forbidden}

Success signal:

{success_signal}
"#,
            constraint = self.constraint,
            priority = self.priority,
            command = self.command,
            forbidden = self.forbidden,
            success_signal = self.success_signal
        )
    }
}

fn missing_integration_command(ticket: &ActionTicket) -> String {
    match (ticket.payment_ready, ticket.outreach_ready) {
        (false, false) => String::from(
            "Add [payment] and [outreach] provider settings to config.toml, then rerun.",
        ),
        (false, true) => {
            String::from("Add [payment] recipient_name and payment_url to config.toml, then rerun.")
        }
        (true, false) => {
            String::from("Add [outreach] to/from provider settings to config.toml, then rerun.")
        }
        (true, true) => String::from("Integrations are configured."),
    }
}
