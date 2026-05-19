use crate::kernel::boundary::QuoteRequest;
use crate::kernel::integration::IntegrationPlan;
use crate::kernel::policy::QuoteDecision;
use crate::kernel::product::ProductLoop;
use crate::kernel::reality::RealitySignal;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct ActionTicket {
    pub command: &'static str,
    pub reason: &'static str,
    pub job_name: String,
    pub quote_price: f64,
    pub required_deposit: f64,
    pub response_deadline_hours: usize,
    pub signal_to_collect: &'static str,
    pub execution_rule: &'static str,
    pub payment_ready: bool,
    pub outreach_ready: bool,
    pub payment_instruction: String,
    pub outreach_instruction: String,
    pub output_path: &'static str,
    pub message_path: &'static str,
    pub payment_request_path: &'static str,
    pub integration_payload_path: &'static str,
    pub email_path: &'static str,
}

impl ActionTicket {
    pub fn from_state(
        request: &QuoteRequest,
        decision: &QuoteDecision,
        signal: &RealitySignal,
        product_loop: &ProductLoop,
        integrations: &IntegrationPlan,
    ) -> Self {
        if signal.is_open_loop() {
            let payment_instruction = payment_instruction(request);
            let outreach_instruction = outreach_instruction(request);
            let integrations_ready = integrations.ready();
            return Self {
                command: if integrations_ready {
                    "send_integrated_deposit_quote_and_capture_payment"
                } else {
                    "configure_payment_and_outreach_integrations"
                },
                reason: if integrations_ready {
                    "empty_pockets_require_cash_signal"
                } else {
                    "cannot_collect_money_without_payment_and_outreach"
                },
                job_name: request.job_name.clone(),
                quote_price: decision.risk_adjusted_quote(),
                required_deposit: required_deposit(request, decision),
                response_deadline_hours: 24,
                signal_to_collect: product_loop.reality_signal,
                execution_rule: if integrations_ready {
                    "do_not_start_work_without_deposit_or_explicit_counter"
                } else {
                    "do_not_contact_customer_until_payment_and_outreach_are_configured"
                },
                payment_ready: integrations.payment_ready,
                outreach_ready: integrations.outreach_ready,
                payment_instruction,
                outreach_instruction,
                output_path: "out/next_action.toml",
                message_path: "out/quote_message.md",
                payment_request_path: "out/payment_request.md",
                integration_payload_path: integrations.payload_path,
                email_path: integrations.email_path,
            };
        }

        Self {
            command: "update_model_from_reality_signal",
            reason: "prediction_error_available",
            job_name: request.job_name.clone(),
            quote_price: decision.risk_adjusted_quote(),
            required_deposit: 0.0,
            response_deadline_hours: 24,
            signal_to_collect: product_loop.reality_signal,
            execution_rule: "update_priors_before_next_quote",
            payment_ready: integrations.payment_ready,
            outreach_ready: integrations.outreach_ready,
            payment_instruction: payment_instruction(request),
            outreach_instruction: outreach_instruction(request),
            output_path: "out/next_action.toml",
            message_path: "out/quote_message.md",
            payment_request_path: "out/payment_request.md",
            integration_payload_path: integrations.payload_path,
            email_path: integrations.email_path,
        }
    }

    pub fn write(&self) -> io::Result<()> {
        if let Some(parent) = Path::new(self.output_path).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(self.output_path, self.to_toml())?;
        fs::write(self.message_path, self.to_message())?;
        fs::write(self.payment_request_path, self.to_payment_request())
    }

    fn to_toml(&self) -> String {
        format!(
            r#"[next_action]
command = "{command}"
reason = "{reason}"
job_name = "{job_name}"
quote_price = {quote_price:.2}
required_deposit = {required_deposit:.2}
response_deadline_hours = {response_deadline_hours}
signal_to_collect = "{signal_to_collect}"
execution_rule = "{execution_rule}"
payment_ready = {payment_ready}
outreach_ready = {outreach_ready}
payment_instruction = "{payment_instruction}"
outreach_instruction = "{outreach_instruction}"
message_path = "{message_path}"
payment_request_path = "{payment_request_path}"
integration_payload_path = "{integration_payload_path}"
email_path = "{email_path}"

[reality_observation_template]
customer_response = "accepted|rejected|countered|paid|ignored"
actual_total_cost = 0.0
actual_cash_received = 0.0
actual_cash_shortfall = 0.0
scrap = false
rework = false
late = false
"#,
            command = self.command,
            reason = self.reason,
            job_name = self.job_name,
            quote_price = self.quote_price,
            required_deposit = self.required_deposit,
            response_deadline_hours = self.response_deadline_hours,
            signal_to_collect = self.signal_to_collect,
            execution_rule = self.execution_rule,
            payment_ready = self.payment_ready,
            outreach_ready = self.outreach_ready,
            payment_instruction = escape_toml_string(&self.payment_instruction),
            outreach_instruction = escape_toml_string(&self.outreach_instruction),
            message_path = self.message_path,
            payment_request_path = self.payment_request_path,
            integration_payload_path = self.integration_payload_path,
            email_path = self.email_path
        )
    }

    fn to_message(&self) -> String {
        format!(
            r#"# Quote: {job_name}

Price: ${quote_price:.2}
Required deposit to begin: ${required_deposit:.2}
Payment: {payment_instruction}
Send via: {outreach_instruction}

This quote is held for {response_deadline_hours} hours.

To move forward, reply with one of:

- accepted
- rejected
- countered
- paid
- ignored

Work does not start until deposit is received or a specific counteroffer is accepted.

Signal being collected:

```text
{signal_to_collect}
```
"#,
            job_name = self.job_name,
            quote_price = self.quote_price,
            required_deposit = self.required_deposit,
            response_deadline_hours = self.response_deadline_hours,
            payment_instruction = self.payment_instruction,
            outreach_instruction = self.outreach_instruction,
            signal_to_collect = self.signal_to_collect
        )
    }

    fn to_payment_request(&self) -> String {
        format!(
            r#"# Payment Request

Job: {job_name}
Deposit due now: ${required_deposit:.2}
Payment: {payment_instruction}

Reply after payment with:

```toml
[reality_observation]
customer_response = "paid"
actual_total_cost = 0.0
actual_cash_received = {required_deposit:.2}
actual_cash_shortfall = 0.0
scrap = false
rework = false
late = false
```
"#,
            job_name = self.job_name,
            required_deposit = self.required_deposit,
            payment_instruction = self.payment_instruction
        )
    }
}

fn required_deposit(request: &QuoteRequest, decision: &QuoteDecision) -> f64 {
    let half_quote = decision.risk_adjusted_quote() * 0.50;
    let cash_gap = decision.average_cash_shortfall();
    let material_floor = request.units * request.base_material_cost_per_unit;

    half_quote.max(cash_gap).max(material_floor)
}

fn payment_instruction(request: &QuoteRequest) -> String {
    request.payment.as_ref().map_or_else(
        || String::from("MISSING_PAYMENT_URL"),
        |payment| {
            let memo = payment.memo.as_deref().unwrap_or(&request.job_name);
            format!(
                "{} via {} memo {}",
                payment.recipient_name, payment.payment_url, memo
            )
        },
    )
}

fn outreach_instruction(request: &QuoteRequest) -> String {
    request.outreach.as_ref().map_or_else(
        || String::from("MISSING_OUTREACH_CHANNEL"),
        |outreach| {
            format!(
                "{} from {} to {}",
                outreach.provider, outreach.from, outreach.to
            )
        },
    )
}

fn escape_toml_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
