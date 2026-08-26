# MTG AI Research Roadmap & Literature Map

**Status:** Living research document  
**Created:** 2026-08-25  
**Primary objective:** Determine which parts of a general-purpose *Magic: The Gathering* simulation and learning system are already solved, which are partially solved, and which remain open research problems.

---

## 1. Project Goal

The purpose of this research track is **not** to build every component from scratch.

Instead:

1. Identify prior work that already solves or substantially addresses each component.
2. Determine the assumptions and limitations of that work.
3. Reuse established ideas, algorithms, formalisms, and open-source implementations where appropriate.
4. Isolate the genuinely unsolved problems.
5. Build the project roadmap around those remaining gaps.

The long-term system under consideration is roughly:

> **Card data / Oracle text → formal representation → rules engine → effect execution → game simulation → telemetry → machine learning / AI agents**

A particularly important research target is whether arbitrary Magic card text can be converted into executable game semantics **without manually hard-coding every individual card**.

---

# 2. Consistent Questions for Every Paper or System

Every paper, framework, engine, benchmark, or open-source project should be evaluated using the same questions.

## Research Map Questions

### 1. What problem does it solve?
What specific part of game modeling, simulation, search, parsing, reasoning, or learning is the work attempting to address?

### 2. What representation does it use?
How are game states, actions, rules, cards, hidden information, effects, or observations represented?

### 3. What does it assume is already known?
Does the work assume that:
- the rules are already formalized?
- card effects are already implemented?
- legal actions can already be generated?
- the game state can already be updated correctly?
- decks are fixed?
- hidden information is represented explicitly?

### 4. What is hard-coded?
Identify every domain-specific component that was manually implemented.

Examples:
- individual cards
- card categories
- phase structure
- combat rules
- legal action generation
- evaluation heuristics
- deck lists
- state features

### 5. Where is human preprocessing required?
What information must a human convert into machine-readable form before the method can operate?

This is especially important for determining whether a system actually **understands card text** or merely operates on previously encoded effects.

### 6. What part of the engine does the work assume away?
Does the research begin only after a functioning simulator already exists?

If so, the simulator itself remains outside the solved scope of the paper.

### 7. How is the work evaluated?
Record:
- benchmark
- card pool
- decks
- number of games
- opponents
- metrics
- computational budget
- baselines

### 8. What has actually been demonstrated?
Separate demonstrated results from broader claims.

Use statuses such as:

- **Solved within stated scope**
- **Substantially solved**
- **Partially solved**
- **Proof of concept**
- **Open**
- **Not addressed**

### 9. What are the important limitations?
Especially note limitations that prevent the result from scaling to full Magic.

### 10. What can this project reuse?
Examples:
- algorithms
- state representations
- formal languages
- action abstractions
- hidden-information techniques
- simulation architecture
- evaluation methodology

### 11. What remains unsolved relative to our project?
This is the most important question.

Each paper should end with a statement of the form:

> **After accepting this work as solved, our project still needs to solve ________.**

---

# 3. High-Level Research Roadmap

## Stage 1 — Formal Game Model

Define the abstract game independently of specific cards where possible.

Examples:

- players
- zones
- objects
- ownership
- control
- turns
- phases and steps
- priority
- actions
- costs
- targets
- stack
- state-based actions
- continuous state
- replacement effects
- triggered events
- hidden information
- random events
- legal action generation

### Current assessment

**General formal game modeling is a mature research area.**

Relevant fields include:

- General Game Playing (GGP)
- Game Description Language (GDL)
- GDL-II
- computational game theory
- automated planning
- logic programming
- extensive-form games
- imperfect-information games

Therefore, the project should first determine which existing formal concepts can be reused rather than inventing a completely new mathematical model.

Magic introduces additional difficulty because cards can alter the rules themselves.

---

## Stage 2 — Rules Engine

Construct or adapt an engine capable of:

- maintaining authoritative game state
- determining legal actions
- applying actions deterministically
- enforcing turn structure
- resolving the stack
- maintaining priority
- handling state-based actions
- generating events
- exposing state to other systems

### Current assessment

**Partially solved in practice, but not automatically solved from natural-language card text.**

Existing Magic engines demonstrate that a working rules engine is achievable.

The main question for this project is whether to:

1. build a new engine, or
2. adapt an existing open-source engine and replace/augment its card-effect implementation layer.

The second option may allow the project to concentrate on the less-solved effect-understanding problem.

---

## Stage 3 — Card / Effect Parsing

Transform Oracle text or another card description into a formal executable representation.

Conceptual pipeline:

```text
Oracle Text
    ↓
Parser
    ↓
Structured Semantic Representation / IR
    ↓
Validation
    ↓
Executable Effect
```

Examples of concepts the representation must eventually express:

- costs
- targets
- conditions
- choices
- quantities
- zones
- object references
- duration
- events
- triggers
- replacement effects
- continuous effects
- dependency relationships
- permissions
- prohibitions
- modifiers

### Current assessment

**This appears to be one of the least-solved components in the Magic-specific literature reviewed so far.**

Most game-playing research assumes that cards and their effects already exist inside the simulator.

This is therefore a major candidate for original research.

---

## Stage 4 — Effect Execution

Take the parsed representation and execute it against the formal game state.

Important distinction:

> **Parsing determines what an effect means. Execution determines how that meaning changes the game.**

The executor should ideally operate on generic primitives rather than card-specific code.

Example:

```text
MOVE_OBJECT
DRAW
DAMAGE
GAIN_LIFE
CREATE_TOKEN
MODIFY_ATTRIBUTE
ADD_ABILITY
COUNTER
SEARCH_ZONE
CHOOSE
TARGET
SCHEDULE_TRIGGER
REPLACE_EVENT
```

A card should ideally compile into combinations of generic operations.

### Current assessment

Likely **partially solved through existing engine architectures**, but the general Oracle-text-to-executable bridge remains an open integration problem.

---

## Stage 5 — Simulation

Once legal actions and effect execution are reliable:

- clone states
- run hypothetical games
- perform rollouts
- search game trees
- sample hidden information
- simulate alternative actions
- produce training trajectories

### Current assessment

**Substantially solved algorithmically once a correct environment exists.**

Relevant established methods include:

- Monte Carlo simulation
- Monte Carlo Tree Search
- determinization
- Information Set MCTS
- rollout policies
- pruning
- search abstractions

The difficult prerequisite is the environment.

---

## Stage 6 — Telemetry

Record actual or simulated games as structured trajectories.

Possible telemetry schema:

```text
state_t
observation_t
legal_actions_t
chosen_action_t
state_t+1
reward / outcome
public_information
private_information
game_metadata
```

Telemetry can connect real Arena games, simulation, analytics, and training.

---

## Stage 7 — Learning / AI

Only after the environment is reliable should learning systems be layered on top.

Possible approaches:

- heuristic agents
- supervised learning
- imitation learning
- reinforcement learning
- self-play
- MCTS-guided policies
- opponent modeling
- causal analysis
- representation learning

### Current assessment

**Many learning techniques already exist.**

The research bottleneck is less likely to be inventing another RL algorithm and more likely to be constructing a sufficiently general, accurate, and scalable Magic environment.

---

# 4. Literature Map

---

## 4.1 Ward & Cowling (2009)

### Citation

Colin D. Ward and Peter I. Cowling.  
**“Monte Carlo Search Applied to Card Selection in Magic: The Gathering.”**  
IEEE Symposium on Computational Intelligence and Games (CIG), 2009, pp. 9–16.  
DOI: https://doi.org/10.1109/CIG.2009.5286501

DBLP:  
https://dblp.org/rec/conf/cig/WardC09

Author-hosted / accessible copy:  
https://www.researchgate.net/publication/224603720_Monte_Carlo_search_applied_to_card_selection_in_Magic_The_Gathering

### Problem addressed

The paper presents Magic as an AI testbed and investigates whether Monte Carlo methods can make useful gameplay decisions in a restricted Magic environment.

The authors emphasize several properties that make Magic interesting for AI:

- imperfect information
- randomness
- very large decision spaces
- opponent modeling
- large numbers of possible cards
- cards that can modify the rules of the game

### What it contributes

The work compares multiple player strategies, including stochastic, rule-based, and Monte Carlo approaches.

Its importance to this project is not that it solves Magic generally.

Its importance is that it demonstrates that **simulation-based decision making is viable once a Magic environment already exists**.

### What it assumes

The game representation and supported card behavior are already implemented.

The paper explicitly considers only a **small subset** of Magic rather than the complete card/rules system.

### Hard-coded / manually represented components

The research depends on a deliberately restricted representation of Magic.

Therefore, it does **not** solve automatic interpretation of arbitrary cards.

### What this means for our project

Accepting the paper's results allows us to treat Monte Carlo search as established prior art.

We do **not** need to rediscover whether repeated simulated play can assist Magic decision-making.

### Remaining gap

> **After accepting the 2009 work, we still need a general mechanism capable of representing and executing far more of Magic without manually implementing every card.**

---

## 4.2 Cowling, Ward & Powley (2012)

### Citation

Peter I. Cowling, Colin D. Ward, and Edward J. Powley.  
**“Ensemble Determinization in Monte Carlo Tree Search for the Imperfect Information Card Game Magic: The Gathering.”**  
IEEE Transactions on Computational Intelligence and AI in Games, 4(4), 2012, pp. 241–257.  
DOI: https://doi.org/10.1109/TCIAIG.2012.2204883

Open repository record:  
https://eprints.whiterose.ac.uk/id/eprint/75050/

### Problem addressed

This work focuses on applying Monte Carlo Tree Search to Magic under:

- incomplete information
- hidden opponent cards
- randomized deck order
- large branching factors

### Major technique: determinization

Unknown information is sampled into possible fully specified game states.

Search is then performed under those sampled possibilities.

The results of multiple determinizations can be combined.

### Additional techniques

The paper investigates:

- pruning strategies
- methods for making random choices more relevant
- finer-grained move generation
- decomposition of move generation into binary yes/no decisions

The authors report stronger play than a basic MCTS implementation in their fixed-deck Magic setting.

### What it assumes

The game itself is already executable.

The simulator already knows:

- legal moves
- state transitions
- card behavior
- game rules

### What it solves for us

It provides established techniques for handling **hidden information and large action spaces during search**.

### What it does not solve

It does not remove the requirement for a manually encoded game environment.

It also does not solve arbitrary Oracle-text interpretation.

### Remaining gap

> **After accepting the 2012 work, we still need the semantic machinery that turns arbitrary Magic cards into the executable environment on which MCTS can operate.**

---

# 5. General Game Playing Literature

Magic-specific research should be combined with work from General Game Playing.

The important conceptual distinction is:

> GGP systems are designed to receive the rules of an unfamiliar game in a **formal machine-readable language** and then reason about the game.

That is already close to part of our desired architecture.

However:

> GGP normally assumes that somebody has already translated the rules into that formal language.

Our project potentially adds the missing upstream transformation:

```text
Natural-language / templated Magic rules text
                ↓
        semantic translation
                ↓
      formal machine language
                ↓
      reasoning / execution
```

---

## 5.1 Genesereth, Love & Pell (2005)

### Citation

Michael Genesereth, Nathaniel Love, and Barney Pell.  
**“General Game Playing: Overview of the AAAI Competition.”**  
AI Magazine, 26(2), 2005, pp. 62–72.  
DOI: https://doi.org/10.1609/aimag.v26i2.1813

GDL introduction:  
https://logic.stanford.edu/ggp/notes/gdl.html

### Core idea

A general game-playing agent should accept a **formal description of a previously unknown game** and play it without a game-specific program being written beforehand.

### Relevance

This establishes an important precedent for our architecture:

**Rules can be data.**

The agent does not necessarily need a bespoke executable program for every individual game.

### Limitation relative to our project

GDL is already formal.

It does not solve the problem:

```text
English-like card text → formal executable rules
```

### Remaining gap

> **The formal reasoning problem is substantially developed; the automatic semantic translation into the formal representation remains the interesting upstream problem.**

---

## 5.2 GDL-II

### Relevant work

GDL-II extends General Game Playing to games involving incomplete information and randomness.

A useful paper is:

**“Reasoning About General Games Described in GDL-II.”**  
AAAI, 2011.

Paper:  
https://ojs.aaai.org/index.php/AAAI/article/download/7944/7803

Overview:  
https://logic.stanford.edu/ggp/chapters/chapter_17.html

### Relevance to Magic

GDL-II introduces explicit machinery for concepts such as:

- random behavior
- private observations / percepts
- incomplete information

These are directly relevant to Magic's hidden hands and shuffled libraries.

### Limitation

Again, the game is assumed to have already been expressed in GDL-II.

### Research implication

We should investigate whether ideas from GDL-II can inform our own internal representation rather than inventing hidden-information semantics from scratch.

---

## 5.3 Świechowski et al. (2015)

### Citation

Maciej Świechowski, HyunSoo Park, Jacek Mańdziuk, and Kyung-Joong Kim.  
**“Recent Advances in General Game Playing.”**  
The Scientific World Journal, 2015, Article 986262.  
DOI: https://doi.org/10.1155/2015/986262

Open access:  
https://pmc.ncbi.nlm.nih.gov/articles/PMC4561326/

### Why it matters

This is a useful survey for understanding the state of GGP rather than reading isolated papers without context.

It reviews work involving:

- Monte Carlo Tree Search
- game-independent reasoning
- knowledge extraction
- formal rule representation
- general video-game playing
- competition benchmarks

### Important architectural lesson

General search methods can operate over many games, but stronger performance often comes from extracting useful knowledge from the game description.

That distinction may become important later:

```text
Card semantics
      ↓
Engine
      ↓
Derived strategic features
      ↓
Search / learning
```

The semantic representation could therefore serve both **execution** and **learning**.

---

# 6. Current Solved / Unsolved Map

| Component | Current Assessment | Notes |
|---|---|---|
| Formal representation of games | **Substantially solved generally** | GGP, GDL, logic programming, planning |
| Complete-information game reasoning | **Mature** | Large existing literature |
| Imperfect-information representation | **Substantially solved generally** | GDL-II, extensive-form games |
| Randomness / hidden state | **Substantially solved generally** | Established formalisms |
| Monte Carlo simulation | **Solved as a technique** | Environment still required |
| MCTS | **Solved as a general technique** | Many refinements exist |
| Search under hidden information | **Substantially developed** | Determinization, ISMCTS, etc. |
| Magic-specific search | **Demonstrated** | Ward/Cowling line of work |
| Full Magic rules engine | **Engineering problem with existing implementations** | Need to inspect open-source engines |
| Generic effect execution primitives | **Partially solved in engines** | Architecture comparison needed |
| Oracle text → semantic representation | **Apparently open / weakly solved** | Major research target |
| Semantic representation → executable effect | **Partially solved conceptually** | Needs Magic-specific design |
| Arbitrary-card support without per-card code | **Open target** | Central project ambition |
| Large-scale simulation | **Engineering problem once engine works** | Parallelism/performance matter |
| Telemetry collection | **Engineering problem** | Schema still to design |
| RL / self-play | **Large existing literature** | Environment quality is bottleneck |
| Generalization to unseen cards | **Open research target** | Requires parser + representation |
| Rule-changing cards | **Major difficulty** | Important test for representation |

---

# 7. Emerging Core Research Question

The current project can be framed more precisely as:

> **Can Magic: The Gathering card text be transformed into a sufficiently expressive formal intermediate representation that a general rules engine can execute previously unseen cards without card-specific implementation?**

This separates the problem into three layers:

```text
              LANGUAGE
Oracle Text ──────────────► Semantic IR
                               │
                               │
                               ▼
              SEMANTICS
                    Game Operations
                               │
                               │
                               ▼
              EXECUTION
                       Rules Engine
```

This is materially different from asking:

> “Can an AI learn to play Magic?”

The learning problem comes **after** the environment problem.

---

# 8. Working Architectural Hypothesis

A useful working architecture is:

```text
                         ┌─────────────────┐
                         │ Comprehensive   │
                         │ Rules Model     │
                         └────────┬────────┘
                                  │
                                  ▼
Card Database ─► Oracle Parser ─► Effect IR
                                  │
                                  ▼
                           Effect Executor
                                  │
                                  ▼
                           Rules Engine
                                  │
                    ┌─────────────┴─────────────┐
                    ▼                           ▼
               Telemetry                   Simulator
                                                │
                                      ┌─────────┴─────────┐
                                      ▼                   ▼
                                    Search                RL
```

The **IR boundary** is especially important.

If designed correctly:

- the parser does not need to manipulate game state directly;
- the engine does not need to understand English;
- the AI does not need to know how Oracle text was parsed;
- the same card representation can be reused by simulation, analytics, search, and learning.

---

# 9. Research Priorities

## Priority A — Formal Game Model

Before creating a new formalism, investigate:

- GDL
- GDL-II
- logic programming
- planning representations
- event calculus
- state-transition systems
- extensive-form games

Goal:

> Determine which pieces can be directly reused and what Magic requires beyond them.

---

## Priority B — Existing Magic Engines

Survey open-source Magic simulators and rules engines.

For each engine, ask:

- How are cards represented?
- Are card implementations declarative or imperative?
- How many cards require custom code?
- What are the engine primitives?
- How are replacement effects represented?
- How are continuous effects represented?
- How are layers implemented?
- How are triggers generated?
- How are targets represented?
- How are costs represented?
- How difficult is adding a new card?
- Can the card layer be replaced without rewriting the engine?

This will determine whether an existing engine can become the execution backend.

---

## Priority C — Effect Parsing

Search literature outside Magic for analogous problems:

- semantic parsing
- natural language → programs
- program synthesis
- executable semantic parsing
- controlled natural language
- semantic role labeling
- knowledge representation
- domain-specific languages
- grammar induction
- neuro-symbolic systems
- text-to-SQL / text-to-code architectures
- compiler intermediate representations

The question is not merely:

> “Can an LLM explain this card?”

It is:

> **Can a system produce a deterministic, validated representation that can safely drive a simulator?**

---

# 10. Proposed Definition of Success

The parser/executor architecture becomes interesting when it can pass increasingly difficult generalization tests.

### Level 0 — Manually encoded effect

The entire card is implemented by a developer.

Not a parsing solution.

### Level 1 — Template recognition

Known Oracle templates map to known primitives.

Example:

```text
"Draw two cards."
→ DRAW(controller, 2)
```

### Level 2 — Compositional parsing

Multiple known structures can be assembled.

Example:

```text
"Destroy target creature. Its controller gains 3 life."
```

becomes multiple linked operations.

### Level 3 — Parameter generalization

Previously unseen combinations of known concepts compile correctly.

### Level 4 — Structural generalization

Previously unseen sentence structures compile from known semantic primitives.

### Level 5 — New-card zero-shot execution

A newly released card can be added to the card database and executed correctly without a developer writing card-specific code.

### Level 6 — Rule-modifying generalization

The system can correctly represent cards that alter permissions, restrictions, replacement behavior, or other rules.

This is likely the hardest level.

---

# 11. Immediate Reading Order

## Read First

### 1. Ward & Cowling — 2009
**Monte Carlo Search Applied to Card Selection in Magic: The Gathering**

Question while reading:

> What parts of Magic did they have to implement manually before Monte Carlo search became possible?

### 2. Cowling, Ward & Powley — 2012
**Ensemble Determinization in Monte Carlo Tree Search for the Imperfect Information Card Game Magic: The Gathering**

Question:

> Which problems are search problems, and which problems are silently delegated to the simulator?

### 3. Genesereth, Love & Pell — 2005
**General Game Playing: Overview of the AAAI Competition**

Question:

> What does a generic game engine require once rules have already been formalized?

### 4. GDL / GDL-II material

Question:

> Could concepts from these languages become part of a Magic intermediate representation?

### 5. Świechowski et al. — 2015
**Recent Advances in General Game Playing**

Question:

> Which general-game techniques should be treated as solved infrastructure rather than new research targets?

---

# 12. Next Literature Categories

The literature map should expand in approximately this order:

1. **Magic-specific AI and simulation**
2. **Existing Magic rules engines**
3. **General Game Playing**
4. **Imperfect-information game AI**
5. **Semantic parsing**
6. **Natural-language-to-program translation**
7. **Program synthesis**
8. **Formal semantics**
9. **Knowledge representation**
10. **Event / rule systems**
11. **Reinforcement-learning environments**
12. **Game telemetry and representation learning**

---

# 13. Working Principle

The project should continually distinguish between:

### A. Problems that are scientifically solved
Use the existing solution.

### B. Problems that are algorithmically solved but require engineering
Implement or adapt them.

### C. Problems that are partially solved
Determine whether existing work can be generalized.

### D. Problems that remain genuinely open
Concentrate original research effort here.

At the moment, the strongest candidate for **D** is:

> **General translation of Magic's card language into executable semantics with minimal or no per-card hard-coding.**

---

# 14. Literature Entry Template

Copy this section for every new paper.

## [Paper Title]

**Authors:**  
**Year:**  
**Venue:**  
**DOI / URL:**  

### Problem

### Representation

### Assumptions

### Hard-coded Components

### Human Preprocessing

### Engine / Environment Assumed

### Method

### Evaluation

### Demonstrated Result

### Limitations

### Reusable Ideas

### Status

- [ ] Solved within scope
- [ ] Substantially solved
- [ ] Partially solved
- [ ] Proof of concept
- [ ] Open
- [ ] Not addressed

### Relevance to Our Architecture

### Remaining Gap

> After accepting this work as solved, our project still needs to solve:

---

# 15. Open Questions

- Can an existing Magic engine provide the rules substrate while we replace its card implementation layer?
- What is the minimum expressive IR capable of representing a useful subset of Oracle text?
- Should the IR be declarative, imperative, event-based, logical, or hybrid?
- How should continuous effects and dependency ordering be represented?
- How should replacement effects be represented?
- How should rule-changing effects be represented?
- Which parts of the Comprehensive Rules belong in engine code versus machine-readable rule data?
- Can Oracle text be treated as a controlled natural language rather than unrestricted English?
- How much of Oracle text can be parsed deterministically with grammars before ML is needed?
- How should parser output be formally validated before execution?
- What benchmark card set should be used to measure parser coverage?
- What constitutes a genuinely "unseen" card for evaluation?
- How should correctness be tested against official Magic rulings and interactions?

---

## Current Research Thesis

**Do not begin by inventing an AI that plays Magic.**

First determine how much existing work can provide:

- the game formalism,
- engine infrastructure,
- hidden-information reasoning,
- simulation,
- search,
- and learning machinery.

Then direct original research toward the missing semantic bridge:

> **Magic rules text → formal executable meaning.**
