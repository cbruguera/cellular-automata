<script>
  import { onMount, onDestroy } from 'svelte'
  import init, { Simulation } from './engine/pkg/ca_engine.js'
  import CaDropdown from './components/CaDropdown.svelte'
  import SettingsPanel from './components/SettingsPanel.svelte'

  let canvas, ctx, sim, wasmMemory, animFrameId
  let running    = false
  let generation = 0
  let population = 0
  let fps        = 0
  let lastFrameTime = 0
  let showPanel  = false
  let loaded     = false
  let error      = null

  let caType      = 'life'
  let ruleB       = [3]
  let ruleS       = [2, 3]
  let boundary    = 'toroidal'
  let neighborhood = 'moore'
  let speed       = 1

  // shown in status bar for 3D+ CAs only
  let showSliceNav = false
  let sliceIndex   = 0
  let sliceMax     = 0

  function buildConfig() {
    return {
      kind: caType,
      shape: [200, 150],
      rule: { birth: ruleB, survival: ruleS },
      boundary,
      neighborhood: { kind: neighborhood, radius: 1 },
    }
  }

  function recreateSim() {
    try {
      running = false
      sim = Simulation.create(JSON.stringify(buildConfig()))
    } catch (e) {
      error = e?.message ?? String(e)
    }
  }

  onMount(async () => {
    try {
      const wasmModule = await init()
      wasmMemory = wasmModule.memory
      sim = Simulation.create(JSON.stringify(buildConfig()))
      ctx = canvas.getContext('2d')
      loaded = true
      resizeCanvas()
      drawFrame()
    } catch (e) {
      error = e?.message ?? String(e)
    }
  })

  onDestroy(() => { if (animFrameId) cancelAnimationFrame(animFrameId) })

  function resizeCanvas() {
    if (!canvas) return
    canvas.width  = canvas.clientWidth
    canvas.height = canvas.clientHeight
  }

  function drawFrame() {
    if (!sim) return
    try {
      if (running) for (let i = 0; i < speed; i++) sim.step()

      sim.render()
      const ptr = sim.pixel_buf_ptr()
      const len = sim.pixel_buf_len()
      const w   = sim.width()
      const h   = sim.height()
      const raw = new Uint8ClampedArray(wasmMemory.buffer, ptr, len)
      const imageData  = new ImageData(raw, w, h)
      const offscreen  = new OffscreenCanvas(w, h)
      const offCtx     = offscreen.getContext('2d')
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

  function togglePlay() { running = !running }
  function step()       { running = false; sim?.step() }

  function handleCaChange(e) {
    caType = e.detail
    recreateSim()
  }

  function handleRuleChange(e) {
    ruleB = e.detail.birth
    ruleS = e.detail.survival
    recreateSim()
  }

  function handleNeighborhoodChange(e) {
    neighborhood = e.detail
    recreateSim()
  }

  function handleBoundaryChange(e) {
    boundary = e.detail
    recreateSim()
  }

  function handleKeydown(e) {
    if (e.key === ' ')          { e.preventDefault(); togglePlay() }
    if (e.key === 'ArrowRight') { e.preventDefault(); step() }
    if (e.key === 'r')          { sim?.randomize() }
    if (e.key === 'Escape')     { showPanel = false }
  }
</script>

<svelte:window on:keydown={handleKeydown} on:resize={resizeCanvas} />

<div class="shell">
  <header class="toolbar">
    <CaDropdown value={caType} on:change={handleCaChange} />
    <div class="spacer"></div>
    <div class="controls">
      <button on:click={step}       disabled={!loaded} title="Step (→)">⏭</button>
      <button on:click={togglePlay} disabled={!loaded} class:active={running} title="Play/Pause (Space)">
        {running ? '⏸' : '⏵'}
      </button>
      <button on:click={() => sim?.randomize()} disabled={!loaded} title="Randomize (R)">↺</button>
    </div>
    <button class="gear" disabled={!loaded} class:active={showPanel}
      on:click={() => showPanel = !showPanel} title="Settings">⚙</button>
  </header>

  <main>
    <canvas bind:this={canvas}></canvas>

    {#if showPanel}
      {#key caType}
        <SettingsPanel
          {speed}
          birthStr={ruleB.join('')}
          survivalStr={ruleS.join('')}
          {boundary}
          {neighborhood}
          on:rulechange={handleRuleChange}
          on:boundarychange={handleBoundaryChange}
          on:neighborhoodchange={handleNeighborhoodChange}
          on:speedchange={e => speed = e.detail}
          on:randomize={() => sim?.randomize()}
          on:clear={() => sim?.clear()}
        />
      {/key}
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
    {#if showSliceNav}
      <span class="dot">·</span>
      <span class="slice-label">z</span>
      <input class="slice-range" type="range" min="0" max={sliceMax} bind:value={sliceIndex} />
      <span class="slice-val">{sliceIndex}/{sliceMax}</span>
    {/if}
  </footer>
</div>

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

  .shell { display: flex; flex-direction: column; height: 100dvh; }

  .toolbar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 16px;
    height: 44px;
    border-bottom: 1px solid #1a1a1a;
    flex-shrink: 0;
  }

  .spacer { flex: 1; }

  .controls { display: flex; gap: 2px; }

  button {
    background: none;
    border: none;
    color: #555;
    font-size: 16px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    transition: color 0.1s;
  }
  button:hover:not(:disabled) { color: #ccc; }
  button:disabled { color: #2a2a2a; cursor: default; }
  button.active   { color: #fff; }
  .gear { font-size: 14px; }

  main {
    flex: 1;
    position: relative;
    overflow: hidden;
    padding: 48px;
  }

  canvas { width: 100%; height: 100%; display: block; }

  .status {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 16px;
    height: 32px;
    border-top: 1px solid #1a1a1a;
    color: #3a3a3a;
    font-size: 11px;
    flex-shrink: 0;
  }
  .dot { color: #222; }

  .slice-label { color: #444; }
  .slice-range { width: 80px; accent-color: #444; }
  .slice-val   { color: #444; min-width: 40px; }

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
  .error-bar button { color: #e06060; font-size: 12px; padding: 2px 6px; }
</style>
