<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    greetMsg = await invoke("greet", { name });
  }
</script>

<section class="hero">
  <div class="logo-container">
    <img src="/flux.png" alt="Flux 3D Logo" class="hero-logo" />
  </div>
  <h1 class="heading">FLUX</h1>
  <p class="subtitle">Premium Cinematic Experience</p>

  <form class="greet-box" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter name for backend test..." bind:value={name} />
    <button type="submit">TEST RUST</button>
  </form>
  
  {#if greetMsg}
    <p class="status-msg">{greetMsg}</p>
  {/if}
</section>

<style>
  .hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    text-align: center;
  }

  h1 {
    font-size: 3rem;
    margin-bottom: 0.5rem;
    color: var(--secondary);
  }

  .logo-container {
    margin-bottom: 2rem;
    perspective: 1000px;
  }

  .hero-logo {
    width: 180px;
    height: 180px;
    object-fit: contain;
    animation: float 6s ease-in-out infinite;
  }

  @keyframes float {
    0%, 100% { transform: translateY(0) rotateY(0); }
    50% { transform: translateY(-20px) rotateY(10deg); }
  }

  .subtitle {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin-bottom: 2rem;
  }

  .greet-box {
    display: flex;
    gap: 12px;
  }

  input {
    background: var(--bg-surface);
    border: 1px solid var(--border);
    color: var(--text-main);
    padding: 10px 16px;
    border-radius: 8px;
    outline: none;
    transition: border-color 0.2s;
  }

  input:focus {
    border-color: var(--primary);
  }

  button {
    background: var(--primary);
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
    letter-spacing: 0.05em;
    transition: transform 0.2s, background 0.2s;
  }

  button:hover {
    background: #9b4dff;
    transform: translateY(-2px);
  }

  .status-msg {
    margin-top: 1rem;
    color: var(--secondary);
    font-family: var(--font-heading);
    font-size: 0.7rem;
  }
</style>
