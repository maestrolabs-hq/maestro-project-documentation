# Idea pipeline

<span class="maestro-status" data-status="exploring">Exploring</span>

An idea is easy to capture and deliberately hard to mistake for a commitment.
Promotion pauses for a human decision at the major gates.

<ol class="maestro-flow" aria-label="Idea-to-production stages">
  <li><strong>Idea</strong><span>Capture without commitment</span></li>
  <li><strong>Brainstorm</strong><span>Clarify purpose and constraints</span></li>
  <li><strong>Grill</strong><span>Challenge value, risk, and assumptions</span></li>
  <li><strong>Requirements</strong><span>Define functional and non-functional needs</span></li>
  <li><strong>Architecture</strong><span>Review only when a boundary requires it</span></li>
  <li><strong>Acceptance</strong><span>Set criteria and definition of done</span></li>
  <li><strong>Split</strong><span>Form initiatives, epics, and tasks</span></li>
  <li><strong>Human approval</strong><span>Turn analysis into commitment</span></li>
  <li><strong>Backlog</strong><span>Prioritize work that is ready</span></li>
  <li><strong>Development</strong><span>Build the approved outcome</span></li>
  <li><strong>Verification</strong><span>Prove the acceptance criteria</span></li>
  <li><strong>Released</strong><span>Publish the verified result</span></li>
</ol>

## Why the gates exist

Discovery can reject, reshape, or retain an idea without creating delivery
pressure. Requirements define the result before decomposition. Architecture is
invited only when the outcome crosses a boundary that needs it. Human approval
is the transition from analysis to commitment.

## Automation boundary

Commands such as `maestro idea add`, `list`, `show`, `promote`, and `delete`
remain a possible interface. This bootstrap documents the model only; it does
not implement those commands or an autonomous role pipeline.
