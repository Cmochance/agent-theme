export const meta = {
  name: 'theme-forge',
  description: 'Per-theme palette tuning: one agent per source image derives a tuned style + builds assets + writes theme.json',
  whenToUse: 'Generate or re-tune agent-theme Codex themes from source images — each theme individually colour-matched (glass from dark tones, accent from the signature colour), not a uniform default.',
  phases: [{ title: 'Forge', detail: 'one agent per image: view → palette → build-theme.sh → theme.json' }],
};

const A = typeof args === 'string' ? JSON.parse(args) : (args || {});
const REPO = A.repo || '/Users/alysechen/alysechen/github/agent-theme';
const THEMES = A.themes || [];
log(`theme-forge: ${THEMES.length} themes`);

const SCHEMA = {
  type: 'object', additionalProperties: false,
  required: ['id', 'accent', 'mood', 'ok'],
  properties: {
    id: { type: 'string' },
    accent: { type: 'string' },
    mood: { type: 'string', description: 'one-line description of the image + chosen palette direction' },
    ok: { type: 'boolean' },
  },
};

function prompt(t) {
  return `You are tuning ONE Codex theme to match its source artwork. Theme id: "${t.id}".

1. Look at the image (Read it): ${t.thumb}
   Note its overall mood, the dominant DARK tone (for glass panels), and the most vivid SIGNATURE colour (for the accent).

2. Decide a cohesive dark-glass palette MATCHED TO THIS IMAGE (do not reuse a generic grey). Rules:
   - glass tiers = the image's dominant dark tone at rising opacity. Provide rgba() strings:
       surface ≈ alpha .50, glass ≈ .60, glassSoft ≈ .52, glassStrong ≈ .78 (darkest/most opaque).
       Use the SAME hue as the image's shadows (e.g. cool blue art → rgba(18,22,30,...); warm → rgba(28,20,13,...)).
   - accent = a vivid, saturated colour pulled from the image (hair, eyes, key light, signature prop). Must read as a link/button colour on dark. Hex.
   - accentSoft = a lighter tint of accent. focus = accent or a slightly brighter variant. Hex.
   - ink = near-white very slightly tinted toward the image temperature (cool → e.g. #eef1f7, warm → #f4ebdf, neutral → #f1ece4).
     ink2 = rgba(ink .74), ink3 = rgba(ink .56), ink4 = rgba(ink .40). Give full rgba() strings.
   - border = rgba(light-tint .14), borderSoft = .07, borderStrong = .26 (tint toward ink).
   - hover = rgba(light-tint .10), selection = .16.
   - scrim = the image's DARKEST shadow tone: scrimTop alpha .26, scrimMid .34, scrimBot .60. rgba() strings.
   - baseColor = the darkest hex (page fallback behind the image).
   - blur = "6px".
   - backgroundPosition: the bg is a 2048² square shown "cover" in a ~1200×800 landscape viewport. Pick so the SUBJECT'S FACE/UPPER BODY stays visible and isn't covered by the centred chat text. Portrait subject high in frame → "center top" or "50% 4%"; subject lower → "center"; busy centre → bias left e.g. "30% top". Choose what frames THIS image best.

${t.build ? `3. Generate assets: run  cd ${REPO} && bash themes/build-theme.sh "${t.src}" ${t.id}
   (creates themes/${t.id}/bg.jpg + preview.jpg).` : `3. This theme already has bg.jpg/preview.jpg — do NOT rebuild them.`}

4. Write themes/${t.id}/theme.json EXACTLY in this shape (valid JSON, your tuned values):
{
  "id": "${t.id}",
  "displayName": { "zh": "${t.zh}", "en": "${t.en}" },
  "background": "bg.jpg",
  "preview": "preview.jpg",
  "backgroundFit": "cover",
  "backgroundPosition": "<your choice>",
  "style": {
    "ink": "...", "ink2": "...", "ink3": "...", "ink4": "...",
    "accent": "...", "accentSoft": "...", "focus": "...",
    "surface": "...", "glass": "...", "glassSoft": "...", "glassStrong": "...",
    "border": "...", "borderSoft": "...", "borderStrong": "...",
    "blur": "6px", "hover": "...", "selection": "...",
    "scrimTop": "...", "scrimMid": "...", "scrimBot": "...",
    "baseColor": "..."
  }
}
Use the Write tool to create themes/${t.id}/theme.json. Then return {id, accent, mood, ok:true}.`;
}

phase('Forge');
const results = await parallel(
  THEMES.map((t) => () => agent(prompt(t), { label: `forge:${t.id}`, phase: 'Forge', schema: SCHEMA })),
);

return { themes: results.filter(Boolean) };
