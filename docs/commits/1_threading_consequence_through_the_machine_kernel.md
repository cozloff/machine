# Machine Kernel Simulation

```toml
[machine_kernel]
name = "Machine Kernel Simulation"
thesis = "Agents persist by selecting policies across physical, informational, and currency networks under uncertainty."

[syntax]
toml = "structure"
ascii_math = "abstraction"
python = "execution"
diagram = "composition"
```

## Refactor Thesis

The refactor turns one linear `main.rs` script into a typed state-transition
system. Each file now names one symbol, owns one transform, and exposes one
piece of structure.

```text
external_state -> typed_boundary -> observation -> physical_consequence
               -> currency_consequence -> policy_decision -> report
```

This bridges the philosophy because the simulator no longer treats a quote as
just arithmetic. It treats a quote as policy selection under constraint:

```text
physics + information + cash + uncertainty -> risk_adjusted_action
```

Math is used here as constraint, not meaning. Randomness exposes possible
consequences. Rust types make the boundaries explicit. The final quote is a
chosen action against those consequences.

## Layer Map

```toml
[layer.0_substrate]
file = "substrate.rs"
symbol = "constant|primitive_transform"
transform = "bound|convert|gate"
structure = "shared_constraint"
ascii_math = "law/constants -> bounded_effect"
rust = "const + fn"

[layer.1_physical_form]
file = "physical_form.rs"
symbol = "material|machine|labor|failure"
transform = "manufacture|scrap|rework|penalize"
structure = "ManufacturingConsequence"
ascii_math = "material + process + failure -> cost_before_financing"
rust = "QuoteRequest + ShockState -> ManufacturingConsequence"

[layer.2_information]
file = "information.rs"
symbol = "config|shock|sample"
transform = "load|parse|observe|sample"
structure = "ShockState"
ascii_math = "world -> TOML -> request; rng -> shocks"
rust = "load_request(path) -> QuoteRequest"

[layer.3_boundary]
file = "boundary.rs"
symbol = "QuoteRequest"
transform = "validate"
structure = "typed_external_state"
ascii_math = "raw_config -> typed_boundary"
rust = "QuoteRequest::validate"

[layer.4_agent]
file = "policy.rs"
symbol = "RiskEngine"
transform = "simulate|decide"
structure = "policy_loop"
ascii_math = "S_t -> policy -> A_t"
rust = "RiskEngine::simulate -> RiskEngine::decide"

[layer.5_network]
file = "mod.rs"
symbol = "module_graph"
transform = "compose"
structure = "crate_adjacency"
ascii_math = "node_i -> use -> node_j"
rust = "pub mod ..."

[layer.6_currency]
file = "currency.rs"
symbol = "cash|profit|shortfall"
transform = "finance|price|measure_runway"
structure = "CurrencyConsequence"
ascii_math = "cost + financing -> profit + cash_shortfall"
rust = "ManufacturingConsequence -> CurrencyConsequence"

[layer.7_policy]
file = "policy.rs"
symbol = "QuoteDecision"
transform = "aggregate|rank|choose"
structure = "decision_surface"
ascii_math = "quote = E[cost] + 1.65*sigma(cost)"
rust = "RiskTrace -> QuoteDecision"

[execution]
file = "execution.rs + main.rs"
symbol = "runtime_trace"
transform = "time|report|compose"
structure = "command_pipeline"
ascii_math = "event -> validate -> state -> policy -> command"
rust = "main"
```

## Adjacency List

```toml
[graph.nodes]
main = "execution entrypoint"
substrate = "shared constants and primitive transforms"
boundary = "typed request boundary"
information = "TOML load and stochastic observations"
physical_form = "manufacturing cost consequence"
currency = "cash and financing consequence"
policy = "risk engine and quote decision"
execution = "timing and report support"

[graph.edges]
main = ["information", "policy", "execution"]
policy = ["boundary", "currency", "execution", "information", "physical_form"]
physical_form = ["boundary", "information", "substrate"]
currency = ["boundary", "physical_form", "substrate"]
information = ["boundary"]
execution = []
boundary = []
substrate = []
```

Equivalent diagram:

```text
                  +-------------+
                  | substrate   |
                  +------+------+ 
                         |
     +----------+--------+---------+
     |          |                  |
+----v-----+ +--v----------+ +-----v------+
| boundary | | information | | execution  |
+----+-----+ +------+-------+ +-----+------+
     |              |               ^
     |              v               |
     |      +-------+-------+       |
     +------> physical_form |       |
            +-------+-------+       |
                    |               |
                    v               |
              +-----+------+        |
              | currency   |        |
              +-----+------+        |
                    |               |
                    v               |
              +-----+------+        |
              | policy     +--------+
              +-----+------+
                    |
                    v
                  report
```

## ASCII Math Views

Symbol/transform/structure:

```text
symbol    = QuoteRequest | ShockState | ManufacturingConsequence | CurrencyConsequence | QuoteDecision
transform = validate | sample | manufacture | finance | decide
structure = typed_state_pipeline
```

Pipeline:

```text
TOML -> parse -> QuoteRequest
QuoteRequest -> validate -> bounded_request
bounded_request + rng -> sample -> ShockState
bounded_request + ShockState -> manufacture -> ManufacturingConsequence
ManufacturingConsequence + cash_terms -> finance -> CurrencyConsequence
CurrencyConsequence*n -> aggregate -> RiskTrace
RiskTrace -> decide -> QuoteDecision
QuoteDecision -> report -> command_output
```

Policy equation:

```text
cost_i = physical_i + financing_i
profit_i = quote_price - cost_i
shortfall_i = max(0, cost_i - cash_on_hand)

E_cost = sum(cost_i)/n
sigma_cost = sqrt(E[cost^2] - E[cost]^2)
risk_adjusted_quote = E_cost + 1.65*sigma_cost
```

Closed loop:

```text
CUR_t -> invest_in_measurement -> INF -> better_policy -> CUR_t+1
```

In the current simulator, `cash_on_hand`, financing cost, and loss probability
are the first currency feedback signals. Later network data can attach supplier
prices, customer conversion probability, machine availability, and quote history
to the same policy loop.

## Python Execution View

The Rust code is the executable system. This Python sketch shows the same
control flow without Rust ownership details:

```python
request = load_request("config.toml")
request.validate()

trace = RiskTrace()

for _ in range(request.simulations):
    shocks = sample(request)
    physical = manufacture(request, shocks)
    currency = finance(request, physical)
    trace.observe(currency)

decision = decide(request, trace)
print_report(request, decision)
```

Layer functions:

```python
def sample(request):
    return ShockState(
        material_return=normal(),
        machine_time_return=normal(),
        scrap=bernoulli(request.scrap_probability),
        rework=bernoulli(request.rework_probability),
        deadline_penalty=bernoulli(request.deadline_penalty_probability),
    )


def manufacture(request, shocks):
    material = material_cost(request, shocks.material_return)
    machine = machine_cost(request, shocks.machine_time_return)
    failure = scrap_cost(shocks.scrap) + rework_cost(shocks.rework)
    penalty = deadline_penalty(shocks.deadline_penalty)
    return ManufacturingConsequence(material, machine, failure, penalty)


def finance(request, physical):
    cost = physical.total_cost_before_financing()
    financing = cost * request.financing_annual_rate * request.days_until_paid / 365
    total = cost + financing
    return CurrencyConsequence(
        total_cost=total,
        profit=request.quote_price - total,
        cash_shortfall=max(0, total - request.cash_on_hand),
    )


def decide(request, trace):
    expected_cost = trace.total_cost / request.simulations
    risk = stdev(trace.costs)
    return QuoteDecision(
        break_even_quote=expected_cost,
        risk_adjusted_quote=expected_cost + 1.65 * risk,
    )
```

## Why This Shape Fits

```toml
[fit]
typed_boundary = "External TOML becomes constrained Rust state."
information = "Random samples are not truth; they are usable state."
physical_form = "Manufacturing physics becomes cost, tolerance, and failure."
currency = "Cost becomes survival pressure through cash shortfall and financing."
policy = "The quote is no longer a number; it is consequence selection."
execution = "The runtime trace shows where time is spent choosing."
```

The important bridge is that every philosophical layer is also a code boundary.
That means the philosophy can evolve without becoming decoration: if you add a
supplier graph, it belongs in a network module; if you add metrology, it belongs
in information or physical form; if you add capital allocation, it belongs in
currency and policy.
