use crate::kernel::action::ActionTicket;
use crate::kernel::boundary::QuoteRequest;
use std::fs;
use std::io;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(Debug)]
pub struct IntegrationPlan {
    pub payment_provider: String,
    pub outreach_provider: String,
    pub payment_ready: bool,
    pub outreach_ready: bool,
    pub payload_path: &'static str,
    pub email_path: &'static str,
    pub stripe_script_path: &'static str,
    pub resend_script_path: &'static str,
}

impl IntegrationPlan {
    pub fn from_request(request: &QuoteRequest) -> Self {
        Self {
            payment_provider: request.payment.as_ref().map_or_else(
                || String::from("missing"),
                |payment| payment.provider.clone(),
            ),
            outreach_provider: request.outreach.as_ref().map_or_else(
                || String::from("missing"),
                |outreach| outreach.provider.clone(),
            ),
            payment_ready: request.payment.is_some(),
            outreach_ready: request.outreach.is_some(),
            payload_path: "out/integration_payload.toml",
            email_path: "out/customer_outreach.eml",
            stripe_script_path: "out/create_stripe_payment_link.sh",
            resend_script_path: "out/send_resend_email.sh",
        }
    }

    pub fn ready(&self) -> bool {
        self.payment_ready && self.outreach_ready
    }

    pub fn write(&self, request: &QuoteRequest, ticket: &ActionTicket) -> io::Result<()> {
        if let Some(parent) = Path::new(self.payload_path).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(self.payload_path, self.to_payload(request, ticket))?;
        fs::write(self.email_path, self.to_email(request, ticket))?;
        fs::write(
            self.stripe_script_path,
            self.to_stripe_script(request, ticket),
        )?;
        fs::write(
            self.resend_script_path,
            self.to_resend_script(request, ticket),
        )?;

        #[cfg(unix)]
        {
            fs::set_permissions(self.stripe_script_path, fs::Permissions::from_mode(0o700))?;
            fs::set_permissions(self.resend_script_path, fs::Permissions::from_mode(0o700))?;
        }

        Ok(())
    }

    fn to_payload(&self, request: &QuoteRequest, ticket: &ActionTicket) -> String {
        format!(
            r#"[integration]
payment_provider = "{payment_provider}"
outreach_provider = "{outreach_provider}"
payment_ready = {payment_ready}
outreach_ready = {outreach_ready}
ready = {ready}

[payment_request]
amount = {deposit:.2}
instruction = "{payment_instruction}"

[outreach]
to = "{to}"
from = "{from}"
subject = "{subject}"
email_path = "{email_path}"
stripe_script_path = "{stripe_script_path}"
resend_script_path = "{resend_script_path}"
"#,
            payment_provider = escape_toml_string(&self.payment_provider),
            outreach_provider = escape_toml_string(&self.outreach_provider),
            payment_ready = self.payment_ready,
            outreach_ready = self.outreach_ready,
            ready = self.ready(),
            deposit = ticket.required_deposit,
            payment_instruction = escape_toml_string(&ticket.payment_instruction),
            to = escape_toml_string(
                request
                    .outreach
                    .as_ref()
                    .map_or("MISSING_OUTREACH_TO", |outreach| outreach.to.as_str())
            ),
            from = escape_toml_string(
                request
                    .outreach
                    .as_ref()
                    .map_or("MISSING_OUTREACH_FROM", |outreach| outreach.from.as_str())
            ),
            subject = escape_toml_string(&subject(request)),
            email_path = self.email_path,
            stripe_script_path = self.stripe_script_path,
            resend_script_path = self.resend_script_path
        )
    }

    fn to_email(&self, request: &QuoteRequest, ticket: &ActionTicket) -> String {
        format!(
            r#"To: {to}
From: {from}
Subject: {subject}

Quote: {job_name}

Price: ${quote_price:.2}
Required deposit to begin: ${deposit:.2}
Payment: {payment_instruction}

This quote is held for {deadline_hours} hours.

Work does not start until deposit is received or a specific counteroffer is accepted.

Please reply with one of: accepted, rejected, countered, paid, ignored.
"#,
            to = request
                .outreach
                .as_ref()
                .map_or("MISSING_OUTREACH_TO", |outreach| outreach.to.as_str()),
            from = request
                .outreach
                .as_ref()
                .map_or("MISSING_OUTREACH_FROM", |outreach| outreach.from.as_str()),
            subject = subject(request),
            job_name = request.job_name,
            quote_price = ticket.quote_price,
            deposit = ticket.required_deposit,
            payment_instruction = ticket.payment_instruction,
            deadline_hours = ticket.response_deadline_hours
        )
    }

    fn to_stripe_script(&self, request: &QuoteRequest, ticket: &ActionTicket) -> String {
        let amount_cents = (ticket.required_deposit * 100.0).round() as i64;
        format!(
            r#"#!/usr/bin/env bash
set -euo pipefail

: "${{STRIPE_SECRET_KEY:?set STRIPE_SECRET_KEY before creating a payment link}}"

mkdir -p out

curl https://api.stripe.com/v1/payment_links \
  -u "$STRIPE_SECRET_KEY:" \
  -d "line_items[0][price_data][currency]"=usd \
  -d "line_items[0][price_data][unit_amount]"={amount_cents} \
  -d "line_items[0][price_data][product_data][name]"="{job_name} deposit" \
  -d "line_items[0][price_data][product_data][description]"="{memo}" \
  -d "line_items[0][quantity]"=1 \
  -d "metadata[job_name]"="{job_name}" \
  -d "metadata[quote_price]"="{quote_price:.2}" \
  -d "metadata[required_deposit]"="{deposit:.2}" \
  -o out/stripe_payment_link.json

printf '\nStripe response written to out/stripe_payment_link.json\n'
printf 'Copy the returned url into [payment].payment_url, rerun sim, then send outreach.\n'
"#,
            amount_cents = amount_cents,
            job_name = shell_escape(&request.job_name),
            memo = shell_escape(&ticket.payment_instruction),
            quote_price = ticket.quote_price,
            deposit = ticket.required_deposit
        )
    }

    fn to_resend_script(&self, request: &QuoteRequest, ticket: &ActionTicket) -> String {
        format!(
            r#"#!/usr/bin/env bash
set -euo pipefail

: "${{RESEND_API_KEY:?set RESEND_API_KEY before sending email}}"

mkdir -p out

curl -X POST "https://api.resend.com/emails" \
  -H "Authorization: Bearer $RESEND_API_KEY" \
  -H "Content-Type: application/json" \
  -H "Idempotency-Key: {idempotency_key}" \
  -d '{{
    "from": "{from}",
    "to": ["{to}"],
    "subject": "{subject}",
    "text": "{body}"
  }}' \
  -o out/resend_email_response.json

printf '\nResend response written to out/resend_email_response.json\n'
"#,
            idempotency_key = json_escape(&format!("{}-{}", request.job_name, ticket.quote_price)),
            from = json_escape(
                request
                    .outreach
                    .as_ref()
                    .map_or("MISSING_OUTREACH_FROM", |outreach| outreach.from.as_str())
            ),
            to = json_escape(
                request
                    .outreach
                    .as_ref()
                    .map_or("MISSING_OUTREACH_TO", |outreach| outreach.to.as_str())
            ),
            subject = json_escape(&subject(request)),
            body = json_escape(&email_body(request, ticket))
        )
    }
}

fn subject(request: &QuoteRequest) -> String {
    request
        .outreach
        .as_ref()
        .and_then(|outreach| outreach.subject.clone())
        .unwrap_or_else(|| format!("Quote for {}", request.job_name))
}

fn escape_toml_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn shell_escape(value: &str) -> String {
    value.replace('"', "\\\"")
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn email_body(request: &QuoteRequest, ticket: &ActionTicket) -> String {
    format!(
        "Quote: {}\n\nPrice: ${:.2}\nRequired deposit to begin: ${:.2}\nPayment: {}\n\nThis quote is held for {} hours.\n\nWork does not start until deposit is received or a specific counteroffer is accepted.\n\nPlease reply with one of: accepted, rejected, countered, paid, ignored.",
        request.job_name,
        ticket.quote_price,
        ticket.required_deposit,
        ticket.payment_instruction,
        ticket.response_deadline_hours
    )
}
