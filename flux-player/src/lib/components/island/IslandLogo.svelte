<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { interpolate } from "flubber";
  import { LETTER_PATHS, SLOT0_COLORS } from "../../constants/island-paths";

  let { active = false } = $props<{ active?: boolean }>();

  // U's exclusive third path — shape is constant, opacity animates.
  const U_PATH2 = LETTER_PATHS[2][2];

  // ── Reactive animated path state ────────────────────────────────────
  let morphPath0  = $state(LETTER_PATHS[0][0]);
  let morphPath1  = $state(LETTER_PATHS[0][1]);
  let path2Opacity = $state(0);
  let slot0Color  = $state(SLOT0_COLORS[0]);

  // ── Animation constants ─────────────────────────────────────────────
  const END_HOLD_MS = 3000;
  const MID_HOLD_MS = 600;
  const MORPH_MS    = 900;

  let letterIdx  = 0;
  let direction  = 1;
  let phase: "holding" | "morphing" = "holding";
  let phaseStart: number | null = null;
  let interp0: ((t: number) => string) | null = null;
  let interp1: ((t: number) => string) | null = null;
  let rafId: number;

  function easeInOut(t: number): number {
    return -(Math.cos(Math.PI * t) - 1) / 2;
  }

  function tick(ts: number): void {
    if (!active) {
      phaseStart = null;
      rafId = requestAnimationFrame(tick);
      return;
    }

    if (phase === "holding") {
      if (phaseStart === null) phaseStart = ts;
      const holdFor = (letterIdx === 0 || letterIdx === 3) ? END_HOLD_MS : MID_HOLD_MS;
      if (ts - phaseStart >= holdFor) {
        if (letterIdx === 3) direction = -1;
        if (letterIdx === 0) direction =  1;

        const nextIdx = letterIdx + direction;
        interp0 = interpolate(LETTER_PATHS[letterIdx][0], LETTER_PATHS[nextIdx][0]);
        interp1 = interpolate(LETTER_PATHS[letterIdx][1], LETTER_PATHS[nextIdx][1]);
        slot0Color = SLOT0_COLORS[nextIdx];
        phase = "morphing";
        phaseStart = ts;
      }
    } else {
      if (phaseStart === null) phaseStart = ts;
      const rawT = Math.min((ts - phaseStart) / MORPH_MS, 1);
      const t = easeInOut(rawT);

      if (interp0) morphPath0 = interp0(t);
      if (interp1) morphPath1 = interp1(t);

      const nextIdx = letterIdx + direction;
      if (nextIdx === 2)      path2Opacity = rawT;
      else if (letterIdx === 2) path2Opacity = 1 - rawT;

      if (rawT >= 1) {
        letterIdx    = nextIdx;
        path2Opacity = letterIdx === 2 ? 1 : 0;
        interp0      = null;
        interp1      = null;
        phase        = "holding";
        phaseStart   = ts;
      }
    }

    rafId = requestAnimationFrame(tick);
  }

  onMount(() => {
    rafId = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafId);
  });
</script>

<div class="island-layer" transition:fade={{ duration: 200 }}>
  <div class="letter-box">
    <svg viewBox="0 0 1024 1024" class="brand-svg" aria-label="FLUX">
      <path d={morphPath0} fill={slot0Color} class="morph-path slot0" />
      <path d={morphPath1} fill="var(--primary)" class="morph-path slot1" />
      <path d={U_PATH2} fill="var(--secondary)" class="morph-path slot2" style="opacity: {path2Opacity};" />
    </svg>
  </div>
</div>

<style>
  .island-layer {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

  .letter-box {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .brand-svg {
    width: 100%;
    height: 100%;
  }

  .morph-path.slot0 {
    transition: fill 0.9s ease;
  }

  .morph-path.slot2 {
    transition: opacity 0.4s ease;
  }
</style>
