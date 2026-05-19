# Machine Kernel Simulation

```toml
[goal_function]
name = "agent_1_reality_optimization"
warning = "nothing changes until an action leaves the model"
```

## Current Move

The simulator now blocks two fake-progress modes:

```text
rerun_same_model -> changing_variables -> no_life_change
report_without_outbound_action -> no_world_signal
```

The correction:

```text
same_config + same_seed -> same_prediction
decision -> out/quote_message.md -> customer -> reply|deposit|silence
decision -> out/next_action.toml -> agent_1 executes -> reality_observation
```

## Rule

```text
changed_reality -> changed_policy
unchanged_reality -> repeat_or_execute_pending_action
```

## Action Artifacts

```toml
[artifacts]
next_action = "out/next_action.toml"
quote_message = "out/quote_message.md"
business_order = "out/business_order.md"
payment_request = "out/payment_request.md"
integration_payload = "out/integration_payload.toml"
email_draft = "out/customer_outreach.eml"
```

`next_action.toml` is the control ticket. `quote_message.md` is the outbound
message. `payment_request.md` is the deposit collection request. `business_order.md`
is the operator instruction: what to do, what not to do, and what signal counts
as success.

Payment rail:

```toml
[payment]
provider = "stripe_payment_links_api"
recipient_name = "Agent 1"
payment_url = "https://example.com/pay/deposit"
memo = "job deposit"
```

If `[payment]` is missing, the business order blocks customer outreach. A quote
that cannot accept a deposit is not a money-moving action.

Outreach channel:

```toml
[outreach]
provider = "resend_email_api"
to = "customer@example.com"
from = "agent1@example.com"
subject = "Quote for job"
```

If `[outreach]` is missing, the business order also blocks customer outreach.
The machine must know where the quote is going before it can operate.

Integration boundary:

```text
config.toml[payment,outreach] -> integration_payload.toml -> provider_call
customer_outreach.eml -> outbound_email_provider
```

No provider API call is made yet. The integration layer prepares the payloads
and blocks unsafe outreach until both payment and outreach are configured.

Provider scripts:

```text
out/create_stripe_payment_link.sh
out/send_resend_email.sh
```

These scripts require `STRIPE_SECRET_KEY` and `RESEND_API_KEY`. They are the
first real-internet handoff point and must be run intentionally.

Provider execution order:

```text
STRIPE_SECRET_KEY=... ./out/create_stripe_payment_link.sh
copy Stripe response url -> config.toml[payment].payment_url
cargo run -- config.integrated.example.toml
RESEND_API_KEY=... ./out/send_resend_email.sh
```

The Stripe API response is saved to `out/stripe_payment_link.json`. The Resend
API response is saved to `out/resend_email_response.json`.

Example modes:

```bash
cargo run
cargo run -- config.integrated.example.toml
cargo run -- config.observed.example.toml
```

The system is not complete when these files are generated; it is complete when
the message leaves the machine and returns a `reality_observation`.

## Loop

```text
agent_1 -> quote_message -> customer_response
customer_response -> cash_event -> reality_observation
reality_observation -> prediction_error -> reward_update
```

## Operator Rule

```text
if open_loop:
  execute_business_order
  do_not_rerun_or_refine_until_customer_response_exists
else:
  update_model_from_reality_signal
```
