<script>
  import { onMount, onDestroy } from 'svelte'
  import init, { Simulation } from './engine/pkg/ca_engine.js'

  let canvas
  let ctx
  let sim
  let wasmMemory
  let animFrameId
  let running = false
  let generation = 0
  let population = 0
  let fps = 0
  let lastFrameTime = 0
  let speed = 1
  let showPanel = false
  let loaded = false
  let error = null

  onMount(async () => {
    try {
      const wasmModule = await init()
      wasmMemory = wasmModule.memory
      sim = Simulation.create(JSON.stringify({
        kind: 'life',
        shape: [200, 150],
      }))
      ctx = canvas.getContext('2d')
      loaded = true
      resizeCanvas()
      drawFrame()
    } catch (e) {
      error = e?.message ?? String(e)
    }
  })

  onDestroy(() => {
    if (animFrameId) cancelAnimationFrame(animFrameId)
  })

  function resizeCanvas() {
    canvas.width = canvas.clientWidth
    canvas.height = canvas.clientHeight
  }

  function drawFrame() {
    if (!sim) return
    try {

    if (running) {
      for (let i = 0; i < speed; i++) sim.step()
    }

    sim.render()

    const ptr = sim.pixel_buf_ptr()
    const len = sim.pixel_buf_len()
    const w = sim.width()
    const h = sim.height()
    const raw = new Uint8ClampedArray(wasmMemory.buffer, ptr, len)
    const imageData = new ImageData(raw, w, h)

    const offscreen = new OffscreenCanvas(w, h)
    const offCtx = offscreen.getContext('2d')
    offCtx.putImageData(imageData, 0, 0)

    ctx.clearRect(0, 0, canvas.width, canvas.height)
    ctx.imageSmoothingEnabled = false
    ctx.drawImage(offscreen, 0, 0, canvas.width, canvas.height)

    generation = Number(sim.generation())
    population = sim.population()

    const now = performance.now()
    if (lastFrameTime) fps = Math.round(1000 / (now - lastFrameTime))
    lastFrameTime = now

      animFrameId = requestAnimationFrame(drawFrame)
    } catch (e) {
      error = e?.message ?? String(e)
    }
  }

  function togglePlay() {
    running = !running
  }

  function step() {
    running = false
    sim.step()
  }

  function reset() {
    running = false
    sim.randomize()
  }

  function clear() {
    running = false
    sim.clear()
  }

  function handleKeydown(e) {
    if (e.key === ' ') { e.preventDefault(); togglePlay() }
    if (e.key === 'ArrowRight') { e.preventDefault(); step() }
    if (e.key === 'r') reset()
    if (e.key === 'Escape') showPanel = false
  }
</script>

<svelte:window on:keydown={handleKeydown} on:resize={resizeCanvas} />

<div class="shell">
  <header class="toolbar">
    <span class="ca-label">Game of Life</span>
    <div class="controls">
      <button on:click={step} disabled={!loaded} title="Step (→)">⏭</button>
      <button on:click={togglePlay} disabled={!loaded} class:active={running} title="Play/Pause (Space)">
        {running ? '⏸' : '⏵'}
      </button>
      <button on:click={reset} disabled={!loaded} title="Randomize (R)">↺</button>
    </div>
    <button class="gear" disabled={!loaded} class:active={showPanel} on:click={() => showPanel = !showPanel} title="Settings">⚙</button>
  </header>

  <main>
    <canvas bind:this={canvas}></canvas>

    {#if showPanel}
      <aside class="panel" on:click|stopPropagation>
        <div class="panel-section">
          <label>Speed <span>{speed}×</span></label>
          <input type="range" min="1" max="20" bind:value={speed} />
        </div>
        <div class="panel-section">
          <button class="full" on:click={clear}>Clear grid</button>
          <button class="full" on:click={reset}>Randomize</button>
        </div>
      </aside>
    {/if}
  </main>

  {#if error}
    <div class="error-bar">
      <span>{error}</span>
      <button on:click={() => error = null}>✕</button>
    </div>
  {/if}

  <footer class="status">
    <span>gen {generation.toLocaleString()}</span>
    <span class="dot">·</span>
    <span>pop {population.toLocaleString()}</span>
    <span class="dot">·</span>
    <span>{running ? fps : '—'} fps</span>
  </footer>
</div>

<!-- close panel on outside click -->
{#if showPanel}
  <div class="overlay" on:click={() => showPanel = false}></div>
{/if}

<style>
  :global(*, *::before, *::after) { box-sizing: border-box; margin: 0; padding: 0; }
  :global(body) {
    background: #0d0d0d;
    color: #ccc;
    font-family: 'SF Mono', 'Fira Code', monospace;
    font-size: 13px;
    height: 100dvh;
    overflow: hidden;
  }

  .shell {
    display: flex;
    flex-direction: column;
    height: 100dvh;
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 16px;
    height: 44px;
    border-bottom: 1px solid #1e1e1e;
    flex-shrink: 0;
  }

  .ca-label {
    color: #888;
    letter-spacing: 0.04em;
    font-size: 12px;
    flex: 1;
  }

  .controls {
    display: flex;
    gap: 4px;
  }

  button {
    background: none;
    border: none;
    color: #666;
    font-size: 16px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    transition: color 0.15s;
  }

  button:hover { color: #ccc; }
  button.active { color: #fff; }

  .gear { font-size: 14px; }

  main {
    flex: 1;
    position: relative;
    overflow: hidden;
  }

  canvas {
    width: 100%;
    height: 100%;
    display: block;
  }

  .panel {
    position: absolute;
    top: 12px;
    right: 12px;
    width: 220px;
    background: #111;
    border: 1px solid #222;
    border-radius: 8px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    z-index: 10;
  }

  .panel-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .panel-section label {
    display: flex;
    justify-content: space-between;
    color: #666;
    font-size: 11px;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .panel-section label span { color: #aaa; }

  input[type=range] {
    width: 100%;
    accent-color: #ccc;
  }

  button.full {
    width: 100%;
    padding: 7px;
    border: 1px solid #222;
    border-radius: 5px;
    font-size: 12px;
    color: #888;
    font-family: inherit;
  }

  button.full:hover { color: #ccc; border-color: #444; }

  .overlay {
    position: fixed;
    inset: 0;
    z-index: 9;
  }

  .status {
    display: flex;
    gap: 8px;
    align-items: center;
    padding: 0 16px;
    height: 32px;
    border-top: 1px solid #1e1e1e;
    color: #444;
    font-size: 11px;
    flex-shrink: 0;
  }

  .dot { color: #2a2a2a; }

  .error-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    background: #1a0a0a;
    border-top: 1px solid #4a1a1a;
    color: #e06060;
    font-size: 12px;
    flex-shrink: 0;
  }

  .error-bar button {
    color: #e06060;
    font-size: 12px;
    padding: 2px 6px;
  }
</style>
