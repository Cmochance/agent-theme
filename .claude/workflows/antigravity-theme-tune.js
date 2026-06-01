export const meta = {
  name: 'antigravity-theme-tune',
  description: 'Iteratively tune the Antigravity background theme against the live app via CDP screenshots',
  whenToUse: 'Refine the translucent/background Antigravity theme to visual perfection: multi-lens critique of live screenshots, then synthesize structural CSS adjustments, looping until scores converge. Edits ONLY the Antigravity-private template; never touches Codex files or theme.json.',
  phases: [
    { title: 'Setup',      detail: 'render CSS from the Antigravity template, inject into live Antigravity, capture baseline shots + light/text scans' },
    { title: 'Critique',   detail: 'parallel lenses score the screenshots and propose template adjustments' },
    { title: 'Synthesize', detail: 'merge adjustments, edit antigravity.template.css, re-render, re-inject, re-capture' },
    { title: 'Final',      detail: 'capture final shots and write a report' },
  ],
};

// ---- config (args override) ----
const REPO   = (args && args.repoRoot)  || '/Users/alysechen/alysechen/github/agent-theme';
const LAB    = `${REPO}/.theme-lab`;
const THEME  = (args && args.theme)     || 'changli';
// The STRUCTURE template (Antigravity-private). The synthesizer edits ONLY this.
const TMPL   = `${LAB}/antigravity.template.css`;
// Knob VALUES come from this theme.json — READ ONLY (shared with Codex; editing
// it would change Codex's look, which is explicitly forbidden).
const THEME_JSON = `${REPO}/themes/${THEME}/theme.json`;
const CSSOUT = `${LAB}/css/${THEME}-ag.css`;
// Antigravity harness: occluded-safe (NO_ACTIVATE) so we never steal the user's focus.
const HARNESS= `AGENT=antigravity NO_ACTIVATE=1 node ${LAB}/cdp.mjs`;
const RENDER = `node ${LAB}/render-antigravity.mjs ${THEME} ${CSSOUT}`;
const SHOTS  = `${LAB}/shots/ag`;
const SRCIMG = `${LAB}/src/${THEME}-768.png`;
const ROUNDS    = (args && args.rounds)    ?? 4;
const THRESHOLD = (args && args.threshold) ?? 88;

const LENSES = [
  { key: 'composition', title: 'Background composition & framing',
    focus: `How the ${THEME} wallpaper reads behind the Antigravity agent UI: the centered chat-input card and the left conversation sidebar sit over the image — is the focal point balanced vs where text lands, is the legibility scrim (#root linear-gradient via --cl-scrim-top/-mid/-bot) balanced top-to-bottom? background-position is fixed by theme.json (do NOT change it). Knobs you MAY change in the template: the #root scrim gradient stops/opacities, any extra overlay rules.` },
  { key: 'readability', title: 'Text readability & contrast',
    focus: `Legibility of every text level over the wallpaper + glass: sidebar items ("New Conversation", "Conversation History", "Settings"), the "Projects" section labels, project/conversation names, the grey timestamps (9d/10d), the chat placeholder ("Ask anything, @ to mention"), the model selector ("Gemini 3.5 Flash"), "Open IDE". Read the textscan json for any dark-on-dark text. Knobs: text-shadow rules in the template (.text-foreground / .text-muted-foreground / placeholder), the --cl-scrim-* darkening, and which --cl-ink level / glass token each surface maps to. Do NOT edit theme.json ink values.` },
  { key: 'glass', title: 'Glass panel quality & cohesion',
    focus: `Frosted-glass surfaces: the conversation sidebar (.bg-sidebar), the centered chat-input card (.bg-card-border frame + .bg-card field), dropdowns/menus/dialogs ([role=menu], .bg-popover). Blur amount, tint opacity, border crispness, drop shadow — do they read as distinct elevated surfaces or muddy/over-transparent? Knobs: the backdrop-filter blur expressions, box-shadow, border rules, and which --cl-glass / --cl-glass-soft / --cl-glass-strong each maps to.` },
  { key: 'accent', title: 'Accent colour cohesion',
    focus: `Antigravity's brand accent is mauve #8839EF; the theme remaps --primary and links/focus/selection to the ${THEME} accent via the accent block. Check: primary buttons, the send button, links, focus ring, ::selection, the model/agent selector chips — do they harmonise with the warm wallpaper or clash? Is on-accent text (--primary-foreground) legible? Knobs: the accent block rules (the accent VALUES come from theme.json; you may adjust which elements get the accent and the --primary-foreground contrast strategy in the template).` },
  { key: 'seams', title: 'Seams, leftover light surfaces & artifacts',
    focus: `Hunt for any surface that did NOT get themed: read the lightscan json for opaque LIGHT rectangles (un-overridden shadcn/vscode tokens still showing Catppuccin Latte light colours), stray hard edges between sidebar↔main, the top bar (40px), the card border frame, scrollbars. Each light surface = a token/selector you must add to the template (override its shadcn --token to a --cl-glass* / transparent, or add a selector rule). Knobs: add token overrides in the html{} block, add module selector rules, border rules.` },
];

const CRITIQUE_SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['dimension', 'score', 'summary', 'issues', 'adjustments'],
  properties: {
    dimension: { type: 'string' },
    score: { type: 'integer', minimum: 0, maximum: 100, description: '100 = perfect, nothing to improve on this dimension' },
    summary: { type: 'string' },
    issues: { type: 'array', items: { type: 'string' }, description: 'concrete problems seen in the screenshots / scans' },
    adjustments: {
      type: 'array',
      items: {
        type: 'object', additionalProperties: false,
        required: ['target', 'change', 'reason'],
        properties: {
          target: { type: 'string', description: 'a knob name like --cl-scrim-bot, a shadcn token like --muted, or a CSS selector to add/modify in antigravity.template.css' },
          change: { type: 'string', description: 'precise new value or rule, e.g. "rgba(13,9,6,.70)" or "add: backdrop-filter blur(10px)"' },
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
  `You tune a live Antigravity theme. Run EXACTLY these shell steps from ${REPO}:
  1. ${RENDER}
  2. ${HARNESS} port   (must print a number; if it prints NONE/NO_PORT, STOP and return ok:false — Antigravity is not running with a debug port, or its window is not visible)
  3. ${HARNESS} inject ${CSSOUT} agent-theme-lab
  4. sleep 1
  5. ${HARNESS} capture ${SHOTS}/r0
  6. ${HARNESS} lightscan ${SHOTS}/r0-light.json
  7. ${HARNESS} textscan ${SHOTS}/r0-text.json
  This writes ${SHOTS}/r0-full.png, ${SHOTS}/r0-sidebar.png, ${SHOTS}/r0-center.png + the two scans.
  If any capture step HANGS or produces no file, the Antigravity window is occluded/minimised — return ok:false with that note.
  Return ok:true and shotPrefix="${SHOTS}/r0".`,
  { label: 'setup', phase: 'Setup', schema: SETUP_SCHEMA },
);

if (!setup || !setup.ok) {
  return { aborted: true, reason: 'Setup failed (Antigravity debug port down or window not visible?). ' + (setup?.notes || ''), setup };
}

let shotPrefix = setup.shotPrefix;
const history = [];

for (let r = 1; r <= ROUNDS; r++) {
  phase('Critique');
  const crits = (await parallel(LENSES.map((L) => () =>
    agent(
      `Critique the CURRENT Antigravity theme screenshots for ONE dimension: "${L.title}".
      Look critically at these images (Read them):
        - ${shotPrefix}-full.png      (whole window)
        - ${shotPrefix}-sidebar.png   (left conversation sidebar, retina)
        - ${shotPrefix}-center.png    (centered chat-input card + main area, retina)
      Also Read these scans for machine-detected problems:
        - ${shotPrefix}-light.json    (opaque LIGHT surfaces = un-themed Catppuccin-Latte leftovers)
        - ${shotPrefix}-text.json     (dark text that may be invisible)
      Reference target art (the source character image): ${SRCIMG}
      The tunable STRUCTURE is ${TMPL} — Read it so your adjustments name REAL selectors/tokens/knobs.
      The knob VALUES resolve from ${THEME_JSON} — Read it to know the concrete colours, but you must NOT edit it (it is shared with Codex).
      Focus ONLY on: ${L.focus}
      Goal: a polished, cohesive, readable warm-dark "${THEME}" wallpaper theme for the Antigravity agent manager where the sidebar + centered chat card read as crisp frosted glass over the image, every text level is comfortably legible, the accent harmonises with the art, and NO light Catppuccin surfaces leak through.
      Score 0-100 (be strict; 100 = truly nothing to improve on THIS dimension). List concrete issues you actually see, and precise adjustments naming knobs/tokens/selectors from ${TMPL}. dimension = "${L.key}".`,
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
    `You are the synthesizer for round ${r} of Antigravity theme tuning. Merge these critique adjustments into the Antigravity STRUCTURE template and re-render.
    Critiques (JSON): ${JSON.stringify(crits.map((c) => ({ dimension: c.dimension, score: c.score, issues: c.issues, adjustments: c.adjustments })))}
    HARD ISOLATION RULES (must obey):
      - Edit ONLY ${TMPL}. Do NOT edit ${THEME_JSON}, any themes/*/theme.json, theme.rs, codex.template.css, or ANY Codex/shipped file. This tuning must not change Codex's look at all.
      - Keep every value expressed via the --cl-* knobs (or color-mix/calc of them) so the theme stays modular/per-theme. Only hardcode rgba when a knob genuinely cannot express it.
    Steps:
    1. Read ${TMPL} and ${THEME_JSON} (for the concrete knob values, read-only).
    2. Decide a coherent merged set of changes. Resolve conflicts between lenses sensibly (readability wants darker scrim, composition wants the image visible — find balance). Prefer adjusting the html{} token overrides, the #root scrim, the module selector rules (blur/shadow/border), and text-shadow rules. Add NEW shadcn/vscode token overrides or NEW selector rules to kill any light leftovers found in the lightscan. Keep changes tasteful and incremental — do NOT overhaul everything at once.
    3. Apply them with the Edit tool to ${TMPL}. Keep it valid CSS. Keep the __INK__/__GLASS__/… placeholders and __HERO__/__ACCENT_BLOCK__/__BASECOLOR__/__POS__/__FIT__ intact (render-antigravity.mjs fills them).
    4. Run: cd ${REPO} && ${RENDER} && ${HARNESS} inject ${CSSOUT} agent-theme-lab && sleep 1 && ${HARNESS} capture ${SHOTS}/r${r} && ${HARNESS} lightscan ${SHOTS}/r${r}-light.json && ${HARNESS} textscan ${SHOTS}/r${r}-text.json
    Return shotPrefix="${SHOTS}/r${r}" and the list of appliedChanges (human-readable).
    Total candidate adjustments this round: ${allAdj.length}.`,
    { label: `synth@r${r}`, phase: 'Synthesize', schema: SYNTH_SCHEMA },
  );
  if (!synth || !synth.shotPrefix) { history.push({ round: r, error: 'synth failed' }); break; }
  shotPrefix = synth.shotPrefix;
}

phase('Final');
const final = await agent(
  `Final review of the tuned Antigravity theme.
  Read ${shotPrefix}-full.png, ${shotPrefix}-sidebar.png, ${shotPrefix}-center.png and the template ${TMPL}.
  Write a concise markdown report to ${LAB}/REPORT-antigravity.md covering: final look per surface (wallpaper/sidebar/chat-card/text/accent/menus), the iteration score history (${JSON.stringify(history)}), the final set of shadcn/vscode token overrides + module rules, and any remaining nits (incl. the code-editor/Monaco caveat — syntax colours stay light unless the user sets Antigravity to a dark colour theme).
  Return perDimension final scores (composition, readability, glass, accent, seams), a one-paragraph summary, and reportPath="${LAB}/REPORT-antigravity.md".`,
  { label: 'final-report', phase: 'Final', schema: FINAL_SCHEMA },
);

return { history, final, lastShotPrefix: shotPrefix };
