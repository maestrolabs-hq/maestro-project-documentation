# Idea pipeline

<span class="maestro-status" data-status="exploring">Exploring</span>

An idea is easy to capture and deliberately hard to mistake for a commitment.
Promotion pauses for a human decision at the major gates.

```text
Idea
  → brainstorm
  → grill
  → functional and non-functional requirements
  → architecture review when needed
  → acceptance criteria and definition of done
  → split into initiatives, epics, and tasks
  → human approval
  → backlog
  → development
  → verification
  → released
```

## Why the gates exist

Discovery can reject, reshape, or retain an idea without creating delivery
pressure. Requirements define the result before decomposition. Architecture is
invited only when the outcome crosses a boundary that needs it. Human approval
is the transition from analysis to commitment.

## Automation boundary

Commands such as `maestro idea add`, `list`, `show`, `promote`, and `delete`
remain a possible interface. This bootstrap documents the model only; it does
not implement those commands or an autonomous role pipeline.
