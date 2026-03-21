<script lang="ts">
  import { onMount } from "svelte";
  import { spring } from "svelte/motion";
  import IslandLogo from "./island/IslandLogo.svelte";
  import IslandControls from "./island/IslandControls.svelte";
  import IslandStatus from "./island/IslandStatus.svelte";
  import IslandMedia from "./island/IslandMedia.svelte";

  // ── Island State ────────────────────────────────────────────────────
  let currentState = $state("idle"); // idle | status | audio | playing | hover
  let previousState = $state("idle");

  // ── Media State Props (Mocked for now) ────────────────────────────────
  let mediaState = $state("idle"); // idle | loading | playing | buffering | paused
  let mediaType = $state("video"); // video | audio
  let bufferingProgress = $state(45);
  let videoTitle = $state("WATCHING...");
  let videoTime = $state("12:34 / 1:45:20");
  let posterSrc = $state("/flux2d.png");

  // ── Adaptive Border & Offline State ──────────────────────────────────
  let isOffline = $state(false);
  let adaptiveTint = $state("var(--island-adaptive-tint)");
  let borderColor = $derived(isOffline ? "#ff0000" : adaptiveTint);

  async function updateAdaptiveTint(src: string) {
    if (!src) return;
    const img = new Image();
    img.crossOrigin = "Anonymous";
    img.src = src;
    img.onload = () => {
      const canvas = document.createElement("canvas");
      canvas.width = 64;
      canvas.height = 64;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;
      ctx.drawImage(img, 0, 0, 64, 64);
      const data = ctx.getImageData(0, 0, 64, 64).data;
      let r = 0, g = 0, b = 0;
      const step = Math.floor(data.length / 4 / 1000) * 4 || 4;
      let count = 0;
      for (let i = 0; i < data.length; i += step) {
        r += data[i];
        g += data[i+1];
        b += data[i+2];
        count++;
      }
      adaptiveTint = `rgb(${Math.floor(r/count)}, ${Math.floor(g/count)}, ${Math.floor(b/count)})`;
    };
  }

  $effect(() => {
    if (mediaState === "playing") {
      updateAdaptiveTint(posterSrc);
    } else {
      adaptiveTint = "var(--island-adaptive-tint)";
    }
  });

  onMount(() => {
    const handleKey = (e: KeyboardEvent) => {
      if (e.key.toLowerCase() === "o") {
        isOffline = !isOffline;
        console.log("Offline mode:", isOffline);
      }
    };
    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  });

  $effect(() => {
    switch (mediaState) {
      case "loading":   currentState = "status"; break;
      case "buffering": currentState = "status"; break;
      case "playing":
        currentState = mediaType === "video" ? "playing" : "audio";
        break;
      case "paused":
      case "idle":
      default:
        currentState = "idle";
        break;
    }
  });

  const size = spring(
    { w: 56, h: 56, r: 28 },
    { stiffness: 0.15, damping: 0.35 }
  );

  $effect(() => {
    switch (currentState) {
      case "idle":    size.set({ w: 56, h: 56, r: 28 });   break;
      case "hover":   size.set({ w: 220, h: 60, r: 30 });  break;
      case "status":  size.set({ w: 320, h: 48, r: 24 });  break;
      case "audio":   size.set({ w: 240, h: 56, r: 28 });  break;
      case "playing": size.set({ w: 200, h: 56, r: 28 });  break;
    }
  });
</script>

<div
  class="dynamic-island"
  style="
    width: {$size.w}px; 
    height: {$size.h}px; 
    border-radius: {$size.r}px; 
    border-color: {borderColor}; 
    --adaptive-tint: {adaptiveTint};
    border-width: 1px; 
    border-style: solid;
  "
  onmouseenter={() => { 
    if (currentState !== "status" && currentState !== "hover") {
      previousState = currentState;
      currentState = "hover";
    }
  }}
  onmouseleave={() => { 
    if (currentState === "hover") {
      currentState = previousState;
    }
  }}
  role="button"
  tabindex="0"
>
  {#if currentState === "idle"}
    <IslandLogo active={true} />
  {:else if currentState === "hover"}
    <IslandControls {mediaState} />
  {:else if currentState === "status"}
    <IslandStatus 
      {mediaState} 
      {bufferingProgress} 
      onClose={() => currentState = "idle"} 
    />
  {:else if currentState === "audio" || currentState === "playing"}
    <IslandMedia 
      type={currentState} 
      {videoTitle} 
      {videoTime} 
      {posterSrc} 
    />
  {/if}
</div>

<style>
  .dynamic-island {
    position: absolute;
    top: 40px;
    left: 50%;
    transform: translateX(-50%);
    background: #000;
    z-index: 20000;
    overflow: hidden;
    cursor: pointer;
    transition: border-color 0.5s ease;
  }
</style>
