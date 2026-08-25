# MTG AI Engine — Research Notes

Context for Claude Code working on the MTG-Frontier-Research repo and the
broader MTG AI engine project. This file consolidates literature findings
and maps them to the project's pipeline stages. It is documentation, not
implementation — treat as background/context, not a spec to execute against.

## Project architecture (for reference)

Card data / Oracle text → formal representation → rules engine → effect
execution → game simulation → telemetry → machine learning / AI agents

**Central open question:** can arbitrary Magic card text be converted into
executable game semantics without manually hard-coding every individual
card? Every source below stops short of answering this — each presupposes
a game model that a human already built by hand. That gap is the project's
actual research frontier.

## Literature — five foundational sources

### 1. Ward & Cowling (2009) — Monte Carlo Search Applied to Card Selection in Magic: The Gathering
- IEEE CIG 2009. DOI: 10.1109/CIG.2009.5286501
- https://www.researchgate.net/publication/224603720_Monte_Carlo_search_applied_to_card_selection_in_Magic_The_Gathering
- **Finding:** Monte Carlo search works for card-selection decisions, but only after a human hand-built the entire simulator, combat rules, resource system, and a small single-colour card pool. Card effects are pre-encoded numeric attributes, not parsed text.
- **Relevance:** validates the search/agent layer works *given* a working simulator; says nothing about producing that simulator from card text. Numeric results (Table 3 win rates) were not independently verifiable — treat as unconfirmed if cited further.

### 2. Cowling, Ward & Powley (2012) — Ensemble Determinization in MCTS for M:TG
- IEEE Trans. Computational Intelligence and AI in Games, Vol. 4 No. 4. DOI: 10.1109/TCIAIG.2012.2204883
- https://eprints.whiterose.ac.uk/id/eprint/75050/1/EnsDetMagic.pdf
- **Finding:** Confirmed numbers — expert-rules player beat reduced-rules player 63.7% of 10,000 games; against 7 human players, won 42.1% of 114 games. Key insight: a stochastic, weaker rollout policy outperforms a rigid expert one as a simulator driver. Ensemble determinization + MCTS handles the hidden hand and shuffling.
- **Relevance:** the search problem (which move to make, given a working simulator) is well-solved by this line of work; it is cleanly separable from — and blind to — the modeling problem (how the simulator itself gets built). Directly reusable for our AI-agent layer once a rules engine exists.

### 3. Genesereth, Love & Pell (2005) — General Game Playing: Overview of the AAAI Competition
- AI Magazine Vol. 26 No. 2. DOI: 10.1609/aimag.v26i2.1813
- https://logic.stanford.edu/publications/genesereth/aaai.pdf
- **Finding:** Defines GDL (Game Description Language) — a Datalog-style formal language for specifying arbitrary finite, deterministic, complete-information games. A generic engine can then *derive* legal moves, state updates, and terminal conditions by logical inference over the axioms, rather than having them hand-coded per game.
- **Relevance:** the "derive the engine from a declarative rule description" pattern is the core architectural idea worth adopting for a Magic IR. Base GDL excludes chance and hidden information — exactly Magic's two hardest features — which is why source #4 matters more.

### 4. GDL-II — Game Description Language, imperfect-information extension
- GDL spec: Stanford Logic Group, Tech Report LG-2006-01 — https://ggp.stanford.edu/readings/gdl_spec.pdf
- GDL-II: Thielscher, AAAI-10 — https://cdn.aaai.org/ojs/7647/7647-13-11177-1-2-20201228.pdf
- Universality proof: Thielscher, IJCAI-11 — https://ijcai.org/Proceedings/11/Papers/189.pdf
- **Finding:** GDL-II adds exactly two keywords to GDL — `random` (models shuffling/drawing) and `sees(R,P)` (models hidden hand / asymmetric information). Thielscher proved GDL-II is universal for finite extensive-form games.
- **Relevance — single most architecturally important source.** `random` and `sees` are ready-made, proven primitives for the two features that defeated the 2009/2012 Magic papers' formalisms. Strong candidate scaffold for the "formal representation" stage: state as fact database, card effects as `next`/`legal` rules, hidden info via `sees`. Gap: still assumes a human hand-writes the axioms, and base GDL lacks native arithmetic (needed for life totals, P/T, mana) — any Magic IR built on this needs an arithmetic extension.

### 5. Świechowski, Park, Mańdziuk & Kim (2015) — Recent Advances in General Game Playing: A Comprehensive Survey
- The Scientific World Journal, Vol. 2015, Article ID 986262. DOI: 10.1155/2015/986262
- **Finding:** Consolidates 2011–2014 GGP research. MCTS/UCT is the dominant approach; catalogs mature enhancements (RAVE, MAST/PAST/FAST, N-grams/LGRP, decaying strategies, early cutoff) and notes Propositional Networks (propnets) as the efficient reasoning substrate for compiled GDL rules, plus MCTS parallelization patterns.
- **Relevance:** tells us the search/agent layer (back half of our pipeline) is solved infrastructure to *integrate*, not research to redo. Frees project effort to concentrate on the front half. Caveat: survey predates deep-RL/AlphaZero-era methods — worth a separate, more recent literature pass before finalizing the AI-agent layer design.

## Cross-cutting takeaway

Every source's "solved boundary" is identical: each begins *after* a
machine-readable game model already exists. None of them convert card
text into that model automatically. The project's novelty has to live at
the front of the pipeline — text → formal representation — since nothing
in the literature attempts that.

## Suggested reading order for anyone new to this project
1. Genesereth, Love & Pell 2005 (GGP framework, easiest entry point)
2. GDL-II material (Thielscher) — the IR candidate
3. Cowling, Ward & Powley 2012 (shows the search layer works, once a model exists)
4. Ward & Cowling 2009 (earlier/narrower version of #3)
5. Świechowski et al. 2015 (survey — read last, as a checklist of what to adopt)

## Data source for card corpus

- Scryfall bulk data API: `https://api.scryfall.com/bulk-data`
  - "Oracle Cards" dataset — one JSON record per unique card (deduplicated
    across printings), current Oracle text, ~25,000+ cards
  - "Rulings" dataset — official rulings keyed by `oracle_id`, useful for
    validating any parser/IR against documented edge cases
  - No API key required for bulk pulls.

## Open questions for the project (not yet researched)

- Deep-RL / self-play literature post-2015 (AlphaZero, MuZero-style
  approaches) — the Świechowski survey stops before this era.
- Whether any existing open-source Magic engine (e.g. Forge, XMage, MTGO's
  internal engine — not yet surveyed) has attempted general effect parsing
  rather than per-card scripting.
- Arithmetic-capable extensions to GDL/GDL-II in the literature, if any
  exist, before designing one from scratch.
