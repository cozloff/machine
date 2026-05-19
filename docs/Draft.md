# Machine

Machine is a long-term B2B software project for precision manufacturing,
materials intelligence, and financial risk simulation.

The practical starting point is simple: help manufacturers make better quoting,
capacity, material, and capital-investment decisions. The long-term technical
edge is deeper: combine manufacturing economics, finance math, materials
science, and quantum/DFT calculations into one decision system.

## Business Thesis

Precision manufacturers often make expensive decisions with incomplete models:
quote prices, machine purchases, material substitutions, supplier choices,
inventory financing, scrap assumptions, and tolerance risk. Those decisions are
financial, operational, and physical at the same time.

Machine should become software that answers questions like:

- What should we quote for this job?
- What is the probability this job loses money?
- Which input most affects profit: material cost, cycle time, scrap, tooling,
  financing, or deadline risk?
- Should we buy, lease, or delay buying a machine?
- Which material gives the best cost, machinability, tolerance, and performance
  tradeoff?
- Can material properties or process difficulty be estimated before expensive
  physical testing?

The first product should not be a full DFT platform. That would be too slow and
too research-heavy for an early business. The first product should be a paid,
useful decision tool for manufacturers. DFT and advanced materials calculations
become the technical moat over time.

## First Product

Build a B2B quote and risk simulator for precision manufacturers.

The first version should model a job with inputs like:

- material type and material price
- material price volatility
- machine hours
- setup time
- labor rate
- machine hourly cost
- tooling cost and tool wear
- scrap probability
- rework probability
- tolerance difficulty
- inspection cost
- deadline penalties
- financing cost
- quote price

The output should be directly useful:

- expected profit
- probability of losing money
- break-even quote price
- recommended risk-adjusted quote price
- expected cash need
- worst-case cash shortfall
- margin sensitivity by input
- machine utilization impact

This is realistic because small and mid-sized manufacturers already care about
quoting, margins, cash flow, and machine utilization. They do not need to buy a
quantum simulation platform on day one. They might pay for better quote
confidence, better margin control, and fewer bad jobs.

## Long-Term Product

The long-term product is a manufacturing decision engine:

```text
material properties
  -> process difficulty
  -> machine time, scrap, tooling, tolerances
  -> cost and schedule risk
  -> quote price, cash flow, ROI, financing, and strategy
```

This can grow into software for:

- machine shops
- aerospace suppliers
- medical device manufacturers
- semiconductor suppliers
- advanced materials startups
- industrial finance and equipment lenders
- procurement teams buying high-spec parts

The deeper moat is connecting physics, manufacturing, and finance in one place.
Most companies treat those as separate spreadsheets, separate departments, or
separate vendors.

## Math Roadmap

The math should be learned by building the product. Every topic below should map
to a feature, model, or customer question.

### Finance Math

- simple and compound interest
- discount rates
- cash flow timing
- working capital
- debt service
- collateral ratios
- ROI
- NPV
- IRR
- payback period
- expected value
- variance and standard deviation
- volatility
- covariance and correlation
- Monte Carlo simulation
- Value at Risk
- expected shortfall
- scenario analysis
- sensitivity analysis
- real options
- capital budgeting
- lease vs buy analysis
- credit risk
- liquidity risk
- margin of safety

Product use: job risk, machine purchase decisions, inventory financing, lender
risk, and cash-flow planning.

### Entrepreneurial Math

- gross margin
- contribution margin
- break-even volume
- price floors
- value-based pricing
- cost-plus pricing
- CAC
- LTV
- churn
- retention
- expansion revenue
- payback period
- sales conversion rates
- pipeline value
- average contract value
- revenue per customer
- unit economics
- burn rate
- runway
- operating leverage
- customer concentration risk
- market sizing
- willingness to pay

Product use: building Machine itself as a real company, pricing the software,
choosing a wedge market, and measuring whether the business model works.

### Economics Math

- supply and demand
- price elasticity
- cross-price elasticity
- opportunity cost
- marginal cost
- marginal revenue
- fixed cost vs variable cost
- capacity constraints
- bottlenecks
- economies of scale
- learning curves
- game theory
- auction and bidding behavior
- adverse selection
- moral hazard
- principal-agent problems
- market power
- switching costs
- transaction costs
- supply shocks
- commodity price dynamics
- queueing economics

Product use: quote strategy, supplier risk, customer pricing, machine capacity,
lead-time premiums, and market timing.

### Manufacturing Math

- cycle time
- takt time
- throughput
- queueing
- bottleneck analysis
- OEE
- setup time
- changeover time
- machine-hour costing
- labor burden
- tooling amortization
- tool wear
- scrap rate
- rework rate
- inspection sampling
- tolerance stack-up
- GD&T basics
- process capability
- Cp and Cpk
- statistical process control
- yield
- first-pass yield
- learning curves
- capacity planning
- job-shop scheduling
- batch sizing
- inventory turns
- safety stock
- lead-time variability
- maintenance intervals
- downtime modeling
- reliability and Weibull analysis

Product use: predicting job cost, schedule risk, capacity impact, scrap risk,
and whether a quote is actually profitable.

### Materials Math

- stress and strain
- Young's modulus
- shear modulus
- bulk modulus
- Poisson's ratio
- hardness
- toughness
- fatigue
- fracture mechanics
- thermal expansion
- heat capacity
- thermal conductivity
- electrical conductivity
- diffusion
- phase transformations
- phase diagrams
- Gibbs free energy
- enthalpy
- entropy
- chemical potential
- defect formation energy
- surface energy
- grain size effects
- dislocation behavior
- corrosion rates
- creep
- wear
- anisotropy
- crystallography
- lattice constants
- Miller indices
- reciprocal lattice
- structure-property relationships
- CALPHAD-style thermodynamic modeling

Product use: connecting material choice to machinability, tool wear, tolerance
risk, thermal behavior, scrap, and part performance.

### Quantum And DFT Math

DFT means Density Functional Theory. It estimates material properties from
electron behavior. It is not the first paid product, but it can become a core
technical advantage for high-value material and process decisions.

Important topics:

- complex numbers
- vector spaces
- inner products
- Hilbert spaces
- matrices
- eigenvalues and eigenvectors
- Hermitian operators
- Fourier series and Fourier transforms
- partial differential equations
- variational calculus
- numerical optimization
- probability amplitudes
- wavefunctions
- electron density
- Schrodinger equation
- Born-Oppenheimer approximation
- Pauli exclusion principle
- spin
- orbitals
- many-body problem
- Hartree approximation
- Hartree-Fock basics
- exchange and correlation
- Hohenberg-Kohn theorems
- Kohn-Sham equations
- exchange-correlation functionals
- local density approximation
- generalized gradient approximation
- hybrid functionals
- self-consistent field iteration
- basis sets
- plane-wave basis methods
- pseudopotentials
- projector augmented wave methods
- k-point sampling
- Brillouin zones
- reciprocal space
- density of states
- band structures
- Fermi level
- phonons
- vibrational modes
- elastic constants from energy-strain curves
- formation energy
- adsorption energy
- defect energy
- surface and interface calculations
- nudged elastic band methods
- ab initio molecular dynamics
- convergence testing
- numerical stability
- high-performance computing
- GPU acceleration

Product use: predicting or enriching material properties, comparing candidate
materials, estimating coatings or surface behavior, screening alloys, and
building a physics-informed material intelligence layer.

## Build Strategy

### Phase 1: Quote Risk Simulator

Turn the existing Monte Carlo risk simulation idea into a manufacturing quote
risk engine. The first implementation can be command-line only. The current
`sim` crate is this first executable slice: it models one manufacturing job and
estimates whether the quote price survives material volatility, machine-time
uncertainty, scrap, rework, deadline penalties, financing cost, and cash timing.

Core models:

- job cost model
- random material price model
- random cycle-time model
- scrap and rework model
- deadline penalty model
- financing cost model
- cash shortfall model
- quote price model
- profit/loss simulation
- sensitivity report

This should produce a useful report for one job:

```text
Expected profit: $4,800
Probability of loss: 18.4%
Break-even quote: $21,300
Recommended quote: $26,900
Most sensitive variable: scrap probability
Worst-case cash shortfall: $7,200
```

The first simulator is intentionally not a full SaaS product and not a full DFT
system. It is the smallest useful model that connects the business pitch to
working code:

```text
manufacturing assumptions
  -> Monte Carlo cost paths
  -> profit, loss probability, quote recommendation, and cash shortfall
```

As the project matures, this same structure can accept better inputs:

- real quote history
- supplier and material price feeds
- machine-specific cycle-time distributions
- material property databases
- DFT-derived or experimentally measured material features
- customer-specific margin and cash constraints

### Phase 2: Customer Discovery

Talk to 10 to 30 people who quote, finance, or operate manufacturing work:

- machine shop owners
- estimators
- operations managers
- industrial accountants
- equipment lenders
- procurement managers
- hardware startup founders

Ask about recent jobs where the quote was wrong, margins disappeared, material
prices moved, a tolerance caused pain, or a machine purchase was debated.

The goal is not compliments. The goal is paid pain.

### Phase 3: Paid Service Before SaaS

Before building a polished SaaS product, sell a small service:

```text
Send me 3 recent jobs and your assumptions.
I will return a quote-risk and margin-risk report.
```

This can be sold manually for a small fee. The service teaches what the software
must automate.

### Phase 4: Data Layer

Build structured data around:

- materials
- vendors
- machine types
- job assumptions
- historical quote outcomes
- inflation and commodity prices
- material properties
- public chemical/material datasets

The current repo already points in this direction with data ingestion, SQLite,
Parquet, PubChem-related commands, and simulation code.

### Phase 5: Materials Intelligence

Add a material comparison layer:

- property lookup
- machinability score
- thermal expansion risk
- corrosion or wear flags
- tolerance difficulty notes
- material substitution comparison
- DFT-derived or database-derived property estimates

At this stage, DFT is used carefully. Prefer public datasets, known computed
materials databases, and validated models before running expensive first-
principles calculations directly.

### Phase 6: Physics-Informed Moat

Build the advanced layer only after the business wedge is working.

Possible advanced features:

- property prediction for uncommon materials
- alloy or coating screening
- surface/interface analysis
- thermal and elastic property estimates
- link DFT-derived features to manufacturing outcomes
- GPU-accelerated simulation pipelines

This is where the company becomes harder to copy.

## Wedge Market

Best first niche:

```text
Small to mid-sized precision machine shops doing high-mix, high-tolerance work.
```

Why:

- quoting errors are expensive
- owners understand margin pain
- job data is structured enough to model
- decisions happen repeatedly
- a useful model can be built without perfect data
- the buyer is closer to the pain

Avoid starting with:

- huge enterprise PLM systems
- full DFT platforms
- generic AI dashboards
- broad manufacturing marketplaces
- long research-only projects

Those can come later, but they are bad first wedges.

## Practical AI Memory

When returning to this project, remember the strategy:

```text
Start with manufacturing quote risk.
Use finance math to model profit and cash-flow risk.
Use manufacturing math to model cycle time, scrap, tooling, and capacity.
Use economics math to model pricing, constraints, and market behavior.
Use entrepreneurial math to build Machine as a real B2B company.
Use materials math to connect material choice to manufacturing outcomes.
Use quantum/DFT math later as a technical moat for advanced materials decisions.
```

The goal is a real business, not just a math notebook. Every mathematical model
should eventually answer a customer decision.