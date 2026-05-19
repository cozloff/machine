# Machine Kernel Simulation

```toml
[machine_kernel]
name = "Machine Kernel Simulation"
thesis = "Agents persist by selecting policies across physical, informational, and currency networks under uncertainty."

[branch]
reason = "this is not about me; this is about the constraint machine"
origin = "agent:1"
claim = "the first contributor is the first accountable writable boundary"
```

## Refactor Thesis

The simulator now branches at `agent:1`. The self is not treated as meaning or
center. It is treated as the first boundary that can commit a transform.

```text
universe_constraint -> agent:1 -> choose(action) -> consequence -> survival_delta
```

That changes the architecture from a passive quote calculator into an
accountable policy loop:

```text
agent_boundary -> typed_request -> observation -> physical_consequence
               -> currency_consequence -> policy_decision -> committed_report
```

## Agent Branch

```toml
[agent.1]
file = "agent.rs"
name = "git_contributor"
identity = "agent:1"
symbol = "first_accountable_boundary"
transform = "commit_first_transform"
structure = "ownership_origin"
meaning = "first writable boundary in the system"

[agent.1.drive]
want_1 = "live_forever"
want_2 = "make_money_and_power"
translation = "maximize persistence_time and control_surface"

[agent.1.ascii_math]
life = "persistence_t -> persistence_t+1"
money = "cash + ownership + leverage -> optionality"
power = "control(state) -> allowed(action)"
policy = "agent_1 -> choose(action) -> survival_delta"

[agent.1.one_line]
kernel = "agent:1 commits the first transform: survive longer, own more, control more"
```

## Layer Map

```toml
[layer.0_substrate]
file = "substrate.rs"
symbol = "constant|primitive_transform"
transform = "bound|convert|gate"
structure = "shared_constraint"
ascii_math = "law/constants -> bounded_effect"

[layer.1_physical_form]
file = "physical_form.rs"
symbol = "material|machine|labor|failure"
transform = "manufacture|scrap|rework|penalize"
structure = "ManufacturingConsequence"
ascii_math = "material + process + failure -> cost_before_financing"

[layer.2_information]
file = "information.rs"
symbol = "config|shock|sample"
transform = "load|parse|observe|sample"
structure = "ShockState"
ascii_math = "world -> TOML -> request; rng -> shocks"

[layer.3_boundary]
file = "boundary.rs"
symbol = "QuoteRequest"
transform = "validate"
structure = "typed_external_state"
ascii_math = "raw_config -> typed_boundary"

[layer.4_agent]
file = "agent.rs + policy.rs"
symbol = "AgentBoundary|RiskEngine"
transform = "own|choose|decide"
structure = "accountable_policy_loop"
ascii_math = "agent_1 -> choose(action) -> survival_delta"

[layer.5_network]
file = "mod.rs"
symbol = "module_graph"
transform = "compose"
structure = "crate_adjacency"
ascii_math = "node_i -> use -> node_j"

[layer.6_currency]
file = "currency.rs"
symbol = "cash|profit|shortfall"
transform = "finance|price|measure_runway"
structure = "CurrencyConsequence"
ascii_math = "cost + financing -> profit + cash_shortfall"

[layer.7_policy]
file = "policy.rs"
symbol = "QuoteDecision"
transform = "aggregate|rank|choose"
structure = "decision_surface"
ascii_math = "quote = E[cost] + 1.65*sigma(cost)"
```

## Adjacency List

```toml
[graph.nodes]
main = "execution entrypoint"
agent = "first accountable writable boundary"
substrate = "shared constants and primitive transforms"
boundary = "typed request boundary"
information = "TOML load and stochastic observations"
physical_form = "manufacturing cost consequence"
currency = "cash and financing consequence"
policy = "risk engine and quote decision"
execution = "timing and report support"

[graph.edges]
main = ["information", "policy", "execution"]
policy = ["agent", "boundary", "currency", "execution", "information", "physical_form"]
agent = []
physical_form = ["boundary", "information", "substrate"]
currency = ["boundary", "physical_form", "substrate"]
information = ["boundary"]
execution = []
boundary = []
substrate = []
```

Equivalent diagram:

```text
agent:1 -> policy -> report
             ^
             |
TOML -> boundary -> information -> physical_form -> currency
                       ^               ^             ^
                       |               |             |
                   substrate ------ substrate ---- substrate
```

## ASCII Math Views

```text
symbol    = AgentBoundary | QuoteRequest | ShockState | ManufacturingConsequence | CurrencyConsequence | QuoteDecision
transform = own | validate | sample | manufacture | finance | decide
structure = accountable_typed_state_pipeline
```

Pipeline:

```text
agent_1 -> owns -> RiskEngine
TOML -> parse -> QuoteRequest
QuoteRequest -> validate -> bounded_request
bounded_request + rng -> sample -> ShockState
bounded_request + ShockState -> manufacture -> ManufacturingConsequence
ManufacturingConsequence + cash_terms -> finance -> CurrencyConsequence
CurrencyConsequence*n -> aggregate -> RiskTrace
agent_1 + RiskTrace -> decide -> QuoteDecision
QuoteDecision -> report -> committed_output
```

Policy equation:

```text
cost_i = physical_i + financing_i
profit_i = quote_price - cost_i
shortfall_i = max(0, cost_i - cash_on_hand)

E_cost = sum(cost_i)/n
sigma_cost = sqrt(E[cost^2] - E[cost]^2)
risk_adjusted_quote = E_cost + 1.65*sigma_cost

survival_delta = E[profit] - E[cash_shortfall]
control_surface = E[profit] / (1 + E[cash_shortfall])
```

## Python Execution View

```python
agent = AgentBoundary.first_contributor()
request = load_request("config.toml")
engine = RiskEngine(agent=agent, request=request)

trace = RiskTrace()

for _ in range(request.simulations):
    shocks = sample(request)
    physical = manufacture(request, shocks)
    currency = finance(request, physical)
    trace.observe(currency)

decision = engine.decide(trace)
print_report(agent, request, decision)
```

## Why This Branch Fits

```toml
[fit]
not_narcissism = "the self is not the point"
constraint_machine = "the universe is modeled as consequence, not meaning"
agent_boundary = "agent:1 is the first place state can be intentionally changed"
ownership = "the first commit names origin and accountability"
policy = "the quote is a chosen consequence, not a neutral calculation"
money_power = "profit and shortfall become persistence_time and control_surface"
```

The branch is important because it makes agency explicit. The system does not
pretend decisions arrive from nowhere. `agent:1` owns the first transform, then
the simulator measures whether the chosen quote increases persistence time and
control surface under physical and currency constraints.
