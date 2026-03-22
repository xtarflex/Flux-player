<script lang="ts">
  let { 
    currentMedia, 
    hasMedia, 
    isLiked = $bindable(false) 
  } = $props<{
    currentMedia: { title: string; duration: string; currentTime: string; poster?: string } | null;
    hasMedia: boolean;
    isLiked?: boolean;
  }>();

  function toggleLike() {
    isLiked = !isLiked;
  }
</script>

<div class="left-section">
  <div class="thumbnail squircle">
    {#if currentMedia?.poster}
      <img src={currentMedia.poster} alt="Poster" />
    {:else}
      <img src="/flux2d.png" alt="Flux Logo" />
    {/if}
  </div>
  <div class="media-info">
    <div class="title">{currentMedia?.title || 'No media playing'}</div>
    <div class="time">{currentMedia?.currentTime || '0:00'} / {currentMedia?.duration || '0:00'}</div>
  </div>
  <button 
    class="icon-btn like-btn {isLiked ? 'liked' : ''}" 
    class:disabled={!hasMedia}
    aria-label="Like" 
    disabled={!hasMedia}
    onclick={toggleLike}
  >
    <svg viewBox="0 0 24 24" fill={isLiked ? "var(--primary)" : "none"} stroke={isLiked ? "var(--primary)" : "currentColor"} stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M20.8 4.6a5.5 5.5 0 0 0-7.7 0l-1.1 1-1.1-1a5.5 5.5 0 0 0-7.8 7.8l1 1 7.9 7.9 7.9-7.9 1-1a5.5 5.5 0 0 0 0-7.8z"/>
    </svg>
    {#if isLiked}
      <div class="heart-burst">
        <span class="p1">♥</span>
        <span class="p2">♥</span>
        <span class="p3">♥</span>
        <span class="p4">♥</span>
      </div>
    {/if}
  </button>
</div>

<style>
  .left-section {
    display: flex;
    align-items: center;
    gap: 16px;
    flex: 1;
    min-width: 0;
  }

  .thumbnail {
    width: 64px;
    height: 64px;
    background: var(--bg-surface);
    overflow: hidden;
    flex-shrink: 0;
    border: 1px solid var(--glass-border-mid);
  }

  .thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    padding: 8px;
  }

  .media-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .title {
    font-size: 14px;
    font-weight: 700;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--text-main);
  }

  .time {
    font-size: 11px;
    color: var(--text-muted);
    font-weight: 500;
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 8px;
    display: flex;
    align-items: center;
    transition: all 0.2s ease;
  }

  .icon-btn:not(:disabled):hover {
    color: var(--text-main);
  }

  .icon-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    pointer-events: none;
  }

  .icon-btn svg { width: 20px; height: 20px; }

  /* Like Button styles */
  .like-btn {
    position: relative;
  }

  @keyframes favorite-pop {
    0% { transform: scale(1); }
    50% { transform: scale(1.3); }
    100% { transform: scale(1); }
  }

  .like-btn.liked {
    color: var(--primary);
    animation: favorite-pop 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  /* Like Burst Particles */
  .heart-burst {
    position: absolute;
    top: 50%;
    left: 50%;
    width: 0;
    height: 0;
    pointer-events: none;
    z-index: -1;
  }

  .heart-burst span {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 14px;
    opacity: 0;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .heart-burst .p1 { 
    background-image: linear-gradient(45deg, var(--primary), var(--secondary)); 
    animation: fly-h1 0.6s cubic-bezier(0.2, 0.8, 0.2, 1) forwards; 
  }
  .heart-burst .p2 { 
    background-image: linear-gradient(135deg, var(--secondary), var(--tertiary, #b07bf0)); 
    animation: fly-h2 0.7s cubic-bezier(0.2, 0.8, 0.2, 1) 0.1s forwards; 
  }
  .heart-burst .p3 { 
    background-image: linear-gradient(225deg, var(--tertiary, #b07bf0), var(--primary)); 
    animation: fly-h3 0.6s cubic-bezier(0.2, 0.8, 0.2, 1) 0.2s forwards; 
  }
  .heart-burst .p4 { 
    background-image: linear-gradient(315deg, var(--primary), #ffffff); 
    animation: fly-h4 0.65s cubic-bezier(0.2, 0.8, 0.2, 1) 0.15s forwards; 
  }

  @keyframes fly-h1 {
    0% { transform: translate(-50%, -50%) scale(0.5); opacity: 1; }
    100% { transform: translate(-250%, -200%) scale(1.2) rotate(-25deg); opacity: 0; }
  }
  @keyframes fly-h2 {
    0% { transform: translate(-50%, -50%) scale(0.3); opacity: 1; }
    100% { transform: translate(150%, -250%) scale(1) rotate(20deg); opacity: 0; }
  }
  @keyframes fly-h3 {
    0% { transform: translate(-50%, -50%) scale(0.4); opacity: 1; }
    100% { transform: translate(-100%, -300%) scale(0.9) rotate(-15deg); opacity: 0; }
  }
  @keyframes fly-h4 {
    0% { transform: translate(-50%, -50%) scale(0.6); opacity: 1; }
    100% { transform: translate(200%, -150%) scale(0.8) rotate(35deg); opacity: 0; }
  }
</style>
