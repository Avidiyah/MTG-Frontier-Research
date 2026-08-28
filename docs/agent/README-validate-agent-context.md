# Validating the agent context metadata

`scripts/python/validate_agent_context.py` is a deterministic,
standard-library-only checker that keeps `docs/findings/index.json`,
`docs/agent/context-map.json`, and the agent entry-point files
(`CLAUDE.md`, `AGENTS.md`, `.github/copilot-instructions.md`,
`docs/agent/agent-contract.md`) structurally consistent with each other and
with the repository they describe.

## Running it

```powershell
python scripts/python/validate_agent_context.py
python -m unittest scripts.python.test_validate_agent_context
```

Prints one JSON document (`{"valid": bool, "errors": [...]}`) to stdout;
exit status is nonzero when invalid. Runs on a clean checkout: no network
access and no built corpus/database required. It also runs in CI on any push
or pull request touching the files it checks
(`.github/workflows/validate-agent-context.yml`).

## What it checks

**`docs/findings/index.json`**

- valid JSON, supported `schema_version`;
- every entry has the required fields (`id`, `path`, `type`, `scope`,
  `date`, `status`, `summary`, `supersedes`, `superseded_by`);
- unique `id`s and unique `path`s;
- every `path` exists, is repository-relative, and cannot escape the
  repository (no absolute paths, no `..`, no drive letters);
- every `supersedes` / `superseded_by` target exists;
- supersession is recorded on both sides: if A supersedes B, B must list A
  in `superseded_by`, and vice versa;
- every `.md` file directly inside `docs/findings/`, `docs/gates/`, or
  `docs/protocol/` (the directories the index's own `purpose` field declares
  it covers) has a matching index entry.

**`docs/agent/context-map.json`**

- everything `python scripts/python/agent_context.py validate` already
  checks (valid JSON, `schema_version`, unique entry/route ids, every
  `ref` resolving to a real entry, non-generated entry paths existing);
- every `validation_commands` item has non-empty `id`/`command` fields and
  unique ids;
- every route's (and the fallback's) `validation[].ref` resolves to a real
  `validation_commands` id -- this was previously unchecked;
- an entry declared `role: generated_local_artifact` is actually covered by
  `.gitignore`, and an entry with any other role is not -- catches a
  generated artifact being represented as required source, and the reverse.

**Agent entry points**

- `CLAUDE.md`, `AGENTS.md`, and `.github/copilot-instructions.md` each still
  contain a reference to `docs/agent/agent-contract.md`;
- `docs/agent/agent-contract.md` still references `docs/current-state.md`
  and `docs/findings/index.json`.

These are substring-presence checks only, not prose comparison -- they catch
a dropped or retargeted pointer, not a wording change.

## What it deliberately does NOT check

- Any current-state research claim (active phase, corpus counts, accuracy
  numbers, which set is newest). Those live in `docs/current-state.md` and
  are not this validator's concern.
- Whether a `type`/`status` value belongs to a fixed enum -- the repository
  does not define one, so the validator only requires them to be non-empty
  strings. Inventing an enum here would block a legitimate new value (e.g.
  a future `type: "audit"`) rather than catch drift.
- Prose quality or wording of any document, including the entry-point files
  and the contract itself.
- Anything under `docs/audits/` -- that is a different, per-set frozen
  artifact category, out of the findings index's declared scope, and must
  stay that way unless the index's own `purpose` field is changed to cover
  it too.
- Semantic correctness of a finding's content, or whether a supersession
  claim is *scientifically* justified -- only that it is recorded
  consistently.

## Adding or changing a findings/gate/protocol document

1. Add or edit the document under `docs/findings/`, `docs/gates/`, or
   `docs/protocol/`.
2. Add or update its entry in `docs/findings/index.json` (`id`, `path`,
   `type`, `scope`, `date`, `status`, `summary`, `supersedes`,
   `superseded_by`). If it supersedes an earlier document, update that
   earlier document's `superseded_by` too -- the validator rejects a
   one-sided relationship.
3. Run `python scripts/python/validate_agent_context.py` before handing off.

## Adding or changing a context-map entry or route

1. Edit `docs/agent/context-map.json` directly (add an `entries[]` item, a
   `task_routes[]` item, or a `validation_commands[]` item as needed).
2. If the new route needs a validation command that does not exist yet, add
   it to `validation_commands` first, then reference its `id`.
3. Run `python scripts/python/validate_agent_context.py`.

A brand-new task class needs no validator code change -- the checks iterate
over whatever routes and entries exist rather than naming any of them.
