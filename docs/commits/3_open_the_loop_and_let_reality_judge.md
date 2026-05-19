# Machine Kernel Simulation

```toml
[machine_kernel]
name = "Machine Kernel Simulation"
thesis = "Agents persist by selecting policies across physical, informational, currency, and reality-feedback networks under uncertainty."

[branch]
origin = "agent:1"
problem = "simulation has no reality_signal"
upgrade = "fake_loop + reality_signal -> accountable_loop"
```

## Current Branch

The simulator now refuses to confuse prediction with truth. A model can sample,
manufacture, finance, decide, and report, but that is still a closed machine
until the world returns evidence.

```text
model_state -> simulate -> decision -> report
```

The next structure is an open feedback loop:

```text
agent_1 -> choose(action) -> world_effect -> reality_signal -> model_update
```

## Reality Signal

```toml
[reality_signal]
file = "reality.rs"
symbol = "RealitySignal"
transform = "observe_real_world|measure_error|update_belief"
structure = "open_feedback_loop"
ascii_math = "Prediction -> Observation -> Error -> ModelUpdate"
rust = "QuoteDecision -> RealitySignal"

[fake_reality]
symbol = "simulated_world"
transform = "sample|manufacture|finance|decide"
structure = "closed_model_loop"
failure = "no external measurement enters the loop"
meaning = "a machine without reality feedback becomes elegant fiction"
```

## Adjacency List

```toml
[graph.nodes]
main = "execution entrypoint"
agent = "first accountable writable boundary"
boundary = "typed request boundary"
information = "TOML load and stochastic observations"
physical_form = "manufacturing cost consequence"
currency = "cash and financing consequence"
policy = "risk engine and quote decision"
reality = "feedback channel from world measurement"
execution = "timing and report support"
substrate = "shared constants and primitive transforms"

[graph.edges]
main = ["information", "policy", "execution"]
policy = ["agent", "boundary", "currency", "execution", "information", "physical_form", "reality"]
reality = ["policy"]
physical_form = ["boundary", "information", "substrate"]
currency = ["boundary", "physical_form", "substrate"]
information = ["boundary"]
agent = []
execution = []
boundary = []
substrate = []
```

## ASCII Math

Closed model loop:

```text
assumption -> simulation -> confidence
```

Open accountable loop:

```text
S_t -> policy -> A_t -> world_t+1 -> y_t+1 -> S_t+1
prediction -> action -> world -> measurement -> error -> update
fake_loop + reality_signal -> accountable_loop
```

Current Rust state:

```text
QuoteDecision -> RealitySignal(OpenLoop)
```

Target Rust state:

```text
Prediction -> RealityObservation -> PredictionError -> ModelUpdate
```

## Python Execution View

```python
agent = AgentBoundary.first_contributor()
request = load_request("config.toml")
engine = RiskEngine(agent=agent, request=request)

trace = engine.simulate()
decision = engine.decide(trace)

prediction = Prediction.from_decision(decision)
observation = observe_world(action=decision)
error = observation - prediction
model = update(model, error)
```

The current implementation creates `RealitySignal::open_loop(decision)` because
there is no external observation source yet. That is intentional. The simulator
should say "no reality has pushed back" instead of pretending its report is
evidence.

## Next Evolution

```toml
[next]
cash_event = "actual cash movement after quote acceptance or rejection"
customer_reply = "accepted|rejected|countered|ignored"
machine_state = "actual runtime, scrap, rework, deadline outcome"
market_response = "observed supplier price and lead time"
model_update = "calibrate predicted cost, profit, and shortfall from observed error"
```

One line:

```text
a simulator predicts; a reality_signal judges; feedback makes it alive
```
