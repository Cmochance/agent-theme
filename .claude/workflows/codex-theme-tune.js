export const meta = {
  name: 'codex-theme-tune',
  description: 'Iteratively tune a Codex background theme against the live app via CDP screenshots',
  whenToUse: 'Refine a translucent/background Codex theme to visual perfection: multi-lens critique of live screenshots, then synthesize CSS-knob adjustments, looping until scores converge.',
  phases: [
    { title: 'Setup',      detail: 'build CSS, inject into live Codex, capture baseline shots' },
    { title: 'Critique',   detail: 'parallel lenses score the screenshots and propose knob adjustments' },
    { title: 'Synthesize', detail: 'merge adjustments, edit template, re-inject, re-capture' },
    { title: 'Final',      detail: 'capture final shots and write a report' },
  ],
};

// ---- config (args override) ----
const REPO   = (args && args.repoRoot)  || '/Users/alysechen/alysechen/github/agent-theme';
const LAB    = `${REPO}/.theme-lab`;
const TMPL   = (args && args.template)  || `${LAB}/css/changli.css.tmpl`;
const CSSOUT = (args && args.cssOut)    || `${LAB}/css/changli.css`;
const HARNESS= `node ${LAB}/cdp.mjs`;
const BUILD  = `node ${LAB}/build-css.mjs ${TMPL} ${CSSOUT}`;
const SHOTS  = `${LAB}/shots`;
const SRCIMG = `${LAB}/src/changli-768.png`;
const ROUNDS    = (args && args.rounds)    ?? 4;
const THRESHOLD = (args && args.threshold) ?? 88;

const LENSES = [
  { key: 'composition', title: 'Background composition & framing',
    focus: 'How the Changli image is framed behind the UI: focal point (her face/hair) vs where chat text sits, background-position/scale, balance of character vs bokeh, whether the hero scrim gradient is balanced. Knobs: the html background shorthand position (e.g. "center top" -> try "38% top"), --cl-scrim-top/-mid/-bot.' },
  { key: 'readability', title: 'Text readability & contrast',
    focus: 'Legibility of every text level over the image and glass: primary heading, sidebar items, suggestion-list rows, composer placeholder/labels, secondary/tertiary/description text. Check WCAG-ish contrast. Knobs: --cl-ink/-2/-3/-4, --cl-scrim-bot (darken busy lower region), text-shadow rules.' },
  { key: 'glass', title: 'Glass panel quality & cohesion',
    focus: 'Frosted-glass panels (sidebar, composer input box, composer footer row, dialogs/menus): blur amount, tint opacity, do they read as distinct surfaces or muddy/over-transparent? Composer input currently looks under-defined. Knobs: --cl-glass/-strong/-soft, --cl-blur, composer border/shadow rules.' },
  { key: 'accent', title: 'Accent colour cohesion',
    focus: 'Warm-orange accent (links, buttons, focus rings, send button, "Full access") vs the amber image — harmonious or clashing/too hot? Knobs: --cl-accent, --cl-accent-soft, ::selection.' },
  { key: 'seams', title: 'Seams, edges & artifacts',
    focus: 'Hard edges or mismatched boundaries between modules (sidebar↔main, composer edges, panel corners), stray opaque rectangles, broken borders, banding in the scrim, the sidebar :after edge fade. Knobs: --cl-border/-soft, module background rules.' },
];

const CRITIQUE_SCHEMA = {
  type: 'object',
  additionalProperties: false,
  required: ['dimension', 'score', 'summary', 'issues', 'adjustments'],
  properties: {
    dimension: { type: 'string' },
    score: { type: 'integer', minimum: 0, maximum: 100, description: '100 = perfect, nothing to improve on this dimension' },
    summary: { type: 'string' },
    issues: { type: 'array', items: { type: 'string' }, description: 'concrete problems seen in the screenshots' },
    adjustments: {
      type: 'array',
      items: {
        type: 'object',
        additionalProperties: false,
        required: ['target', 'change', 'reason'],
        properties: {
          target: { type: 'string', description: 'knob name like --cl-scrim-bot, or a CSS selector to add/modify' },
          change: { type: 'string', description: 'precise new value or rule, e.g. "rgba(13,9,6,.70)" or "add: border:1px solid var(--cl-border)"' },
          reason: { type: 'string' },
        },
      },
    },
  },
};

const SETUP_SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['ok', 'shotPrefix', 'notes'],
  properties: {
    ok: { type: 'boolean' },
    shotPrefix: { type: 'string', description: 'absolute path prefix of captured shots, without the -full.png suffix' },
    notes: { type: 'string' },
  },
};

const SYNTH_SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['shotPrefix', 'appliedChanges', 'notes'],
  properties: {
    shotPrefix: { type: 'string', description: 'absolute path prefix of the NEW shots captured after re-injecting' },
    appliedChanges: { type: 'array', items: { type: 'string' } },
    notes: { type: 'string' },
  },
};

const FINAL_SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['summary', 'reportPath', 'perDimension'],
  properties: {
    summary: { type: 'string' },
    reportPath: { type: 'string' },
    perDimension: { type: 'array', items: {
      type: 'object', additionalProperties: false,
      required: ['dimension', 'score'],
      properties: { dimension: { type: 'string' }, score: { type: 'integer' } },
    } },
  },
};

// ============================ run ============================
phase('Setup');
const setup = await agent(
  `You tune a live Codex theme. Run EXACTLY these shell steps from ${REPO}:
  1. ${BUILD}
  2. ${HARNESS} port   (must print a number; if it prints NONE/NO_PORT, STOP and return ok:false — Codex is not running with a debug port)
  3. ${HARNESS} inject ${CSSOUT}
  4. sleep 1
  5. ${HARNESS} capture ${SHOTS}/r0
  This writes ${SHOTS}/r0-full.png, ${SHOTS}/r0-sidebar.png, ${SHOTS}/r0-composer.png.
  Return ok:true and shotPrefix="${SHOTS}/r0".`,
  { label: 'setup', phase: 'Setup', schema: SETUP_SCHEMA },
);

if (!setup || !setup.ok) {
  return { aborted: true, reason: 'Setup failed (no live Codex debug port?). ' + (setup?.notes || ''), setup };
}

let shotPrefix = setup.shotPrefix;
const history = [];

for (let r = 1; r <= ROUNDS; r++) {
  phase('Critique');
  const crits = (await parallel(LENSES.map((L) => () =>
    agent(
      `Critique the CURRENT Codex theme screenshots for ONE dimension: "${L.title}".
      Look critically at these images (Read them):
        - ${shotPrefix}-full.png      (whole window)
        - ${shotPrefix}-sidebar.png   (left sidebar, retina)
        - ${shotPrefix}-composer.png  (composer + suggestion list, retina)
      Reference target art (the source character image): ${SRCIMG}
      Current tunable knobs are the :root block of ${TMPL} — Read it so your adjustments name REAL knobs.
      Focus ONLY on: ${L.focus}
      The goal is a polished, cohesive, readable warm-dark "Changli" wallpaper theme where each module's background matches perfectly and text of every type is comfortably legible.
      Score 0-100 (be strict; 100 means truly nothing to improve on THIS dimension). List concrete issues you actually see, and precise adjustments naming knobs from the :root block (or a selector+rule to add). dimension = "${L.key}".`,
      { label: `crit:${L.key}@r${r}`, phase: 'Critique', schema: CRITIQUE_SCHEMA },
    ),
  ))).filter(Boolean);

  if (!crits.length) { history.push({ round: r, error: 'no critiques' }); break; }
  const scores = crits.map((c) => c.score);
  const minScore = Math.min(...scores);
  const avgScore = Math.round(scores.reduce((a, b) => a + b, 0) / scores.length);
  history.push({ round: r, minScore, avgScore, dims: crits.map((c) => ({ d: c.dimension, s: c.score })) });
  log(`round ${r}: min=${minScore} avg=${avgScore} [${crits.map((c) => c.dimension + ':' + c.score).join(', ')}]`);

  if (minScore >= THRESHOLD) { log(`converged at round ${r} (min ${minScore} >= ${THRESHOLD})`); break; }
  if (r === ROUNDS) break; // last round: critique only, no further synth

  phase('Synthesize');
  const allAdj = crits.flatMap((c) => c.adjustments.map((a) => ({ dim: c.dimension, ...a })));
  const synth = await agent(
    `You are the synthesizer for round ${r} of Codex theme tuning. Merge these critique adjustments into the theme template and re-render.
    Critiques (JSON): ${JSON.stringify(crits.map((c) => ({ dimension: c.dimension, score: c.score, issues: c.issues, adjustments: c.adjustments })))}
    Steps:
    1. Read ${TMPL}.
    2. Decide a coherent merged set of changes. Resolve conflicts between lenses sensibly (e.g. readability wants darker scrim, composition wants the image visible — find balance). Prefer adjusting the :root knob values; only add/modify module rules when a knob can't express it. Keep changes tasteful and incremental — do NOT overhaul everything at once.
    3. Apply them with the Edit tool to ${TMPL} (edit :root knob values and/or module rules). Keep it valid CSS. Do not touch the url(asset:...) placeholders.
    4. Run: cd ${REPO} && ${BUILD} && ${HARNESS} inject ${CSSOUT} && sleep 1 && ${HARNESS} capture ${SHOTS}/r${r}
    Return shotPrefix="${SHOTS}/r${r}" and the list of appliedChanges (human-readable).
    Total candidate adjustments this round: ${allAdj.length}.`,
    { label: `synth@r${r}`, phase: 'Synthesize', schema: SYNTH_SCHEMA },
  );
  if (!synth || !synth.shotPrefix) { history.push({ round: r, error: 'synth failed' }); break; }
  shotPrefix = synth.shotPrefix;
}

phase('Final');
const final = await agent(
  `Final review of the tuned Codex theme.
  Read ${shotPrefix}-full.png, ${shotPrefix}-sidebar.png, ${shotPrefix}-composer.png and the template ${TMPL}.
  Write a concise markdown report to ${LAB}/REPORT.md covering: final look per module (background/sidebar/composer/text/accent), the iteration score history (${JSON.stringify(history)}), the final :root knob values, and any remaining nits.
  Return perDimension final scores (composition, readability, glass, accent, seams), a one-paragraph summary, and reportPath="${LAB}/REPORT.md".`,
  { label: 'final-report', phase: 'Final', schema: FINAL_SCHEMA },
);

return { history, final, lastShotPrefix: shotPrefix };
