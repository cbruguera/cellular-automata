<script>
  import { createEventDispatcher } from 'svelte'
  import { PATTERNS } from '../lib/patterns.js'

  export let speed        = 1
  export let birthStr     = '3'
  export let survivalStr  = '23'
  export let boundary     = 'toroidal'
  export let neighborhood = 'moore'
  export let gridW        = 200
  export let gridH        = 150

  const dispatch = createEventDispatcher()
  let tab = 'rule'
  let customRle = ''
  let inputW = gridW
  let inputH = gridH

  function applyRule() {
    const parse = s => [...s].map(Number).filter(n => Number.isInteger(n) && n >= 0 && n <= 9)
    dispatch('rulechange', { birth: parse(birthStr), survival: parse(survivalStr) })
  }

  function onKey(e) { if (e.key === 'Enter') applyRule() }

  function loadPattern(rle) {
    dispatch('loadpattern', rle)
  }
</script>

<aside class="panel">
  <nav class="tabs">
    <button class:active={tab === 'rule'}     on:click={() => tab = 'rule'}>Rule</button>
    <button class:active={tab === 'patterns'} on:click={() => tab = 'patterns'}>Patterns</button>
    <button class:active={tab === 'view'}     on:click={() => tab = 'view'}>View</button>
  </nav>

  {#if tab === 'rule'}
    <div class="section">
      <div class="row">
        <label for="birth">B</label>
        <input id="birth" type="text" bind:value={birthStr} on:keydown={onKey} maxlength="9" />
      </div>
      <div class="row">
        <label for="survival">S</label>
        <input id="survival" type="text" bind:value={survivalStr} on:keydown={onKey} maxlength="9" />
      </div>
      <button class="full" on:click={applyRule}>Apply rule</button>
    </div>

    <div class="sep"></div>

    <div class="section">
      <div class="row">
        <label for="neighborhood">Neighborhood</label>
        <select id="neighborhood" bind:value={neighborhood}
          on:change={() => dispatch('neighborhoodchange', neighborhood)}>
          <option value="moore">Moore</option>
          <option value="vonneumann">Von Neumann</option>
        </select>
      </div>
      <div class="row">
        <label for="boundary">Boundary</label>
        <select id="boundary" bind:value={boundary}
          on:change={() => dispatch('boundarychange', boundary)}>
          <option value="toroidal">Toroidal</option>
          <option value="dead">Dead</option>
        </select>
      </div>
    </div>

    <div class="sep"></div>

    <div class="section">
      <div class="row">
        <label for="speed">Speed</label>
        <span class="val">{speed}×</span>
      </div>
      <input id="speed" type="range" min="1" max="20" bind:value={speed}
        on:input={() => dispatch('speedchange', speed)} />
    </div>

    <div class="sep"></div>

    <div class="section actions">
      <button class="full" on:click={() => dispatch('randomize')}>Randomize</button>
      <button class="full" on:click={() => dispatch('clear')}>Clear</button>
    </div>

    <div class="sep"></div>

    <div class="section">
      <div class="dim-label">Grid size</div>
      <div class="row">
        <label for="grid-w">W</label>
        <input id="grid-w" type="number" min="10" max="2000" bind:value={inputW} />
      </div>
      <div class="row">
        <label for="grid-h">H</label>
        <input id="grid-h" type="number" min="10" max="2000" bind:value={inputH} />
      </div>
      <button class="full" on:click={() => dispatch('gridsizechange', { width: +inputW, height: +inputH })}>
        Apply
      </button>
    </div>
  {/if}

  {#if tab === 'patterns'}
    <div class="pattern-list">
      {#each PATTERNS as group}
        <div class="group-label">{group.group}</div>
        {#each group.items as pattern}
          <button class="pattern-item" on:click={() => loadPattern(pattern.rle)}>
            <span class="pattern-name">{pattern.name}</span>
            <span class="pattern-meta">{pattern.meta}</span>
          </button>
        {/each}
      {/each}
    </div>

    <div class="sep"></div>

    <div class="section">
      <div class="rle-label">Custom RLE</div>
      <textarea
        bind:value={customRle}
        placeholder="Paste RLE here…"
        rows="3"
        spellcheck="false"
      ></textarea>
      <button class="full" on:click={() => customRle.trim() && loadPattern(customRle.trim())}>
        Load
      </button>
    </div>
  {/if}

  {#if tab === 'view'}
    <div class="section">
      <div class="row">
        <label for="colors">Colors</label>
        <select id="colors" disabled>
          <option>Classic</option>
          <option>Neon</option>
          <option>Heatmap</option>
        </select>
      </div>
    </div>

    <div class="sep"></div>

    <div class="section dim">
      <div class="dim-label">3D / N-D</div>
      <div class="row">
        <label for="slice-axis">Slice axis</label>
        <select id="slice-axis" disabled>
          <option>Z</option><option>X</option><option>Y</option>
        </select>
      </div>
      <div class="row">
        <label for="depth">Depth</label>
        <span class="val">—</span>
      </div>
      <input id="depth" type="range" disabled />
    </div>
  {/if}
</aside>

<style>
  .panel {
    position: absolute;
    top: 12px;
    right: 12px;
    width: 230px;
    background: #111;
    border: 1px solid #1e1e1e;
    border-radius: 8px;
    z-index: 10;
    overflow: hidden;
    max-height: calc(100% - 24px);
    overflow-y: auto;
  }

  .tabs {
    display: flex;
    border-bottom: 1px solid #1e1e1e;
    position: sticky;
    top: 0;
    background: #111;
    z-index: 1;
  }
  .tabs button {
    flex: 1;
    padding: 9px 4px;
    background: none;
    border: none;
    border-bottom: 1px solid transparent;
    margin-bottom: -1px;
    color: #3a3a3a;
    font-family: inherit;
    font-size: 10px;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    cursor: pointer;
  }
  .tabs button.active { color: #999; border-bottom-color: #999; }
  .tabs button:hover:not(.active) { color: #666; }

  .section { padding: 12px 14px; }
  .section.actions { display: flex; gap: 8px; }
  .sep { border-top: 1px solid #191919; }

  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 9px;
  }
  .row:last-child { margin-bottom: 0; }

  label { color: #4a4a4a; font-size: 11px; }
  .val  { color: #555; font-size: 11px; }

  input[type=text] {
    width: 58px;
    background: #0d0d0d;
    border: 1px solid #1e1e1e;
    border-radius: 4px;
    color: #bbb;
    font-family: inherit;
    font-size: 12px;
    padding: 3px 7px;
    text-align: center;
  }
  input[type=text]:focus { outline: none; border-color: #383838; }

  input[type=number] {
    width: 70px;
    background: #0d0d0d;
    border: 1px solid #1e1e1e;
    border-radius: 4px;
    color: #bbb;
    font-family: inherit;
    font-size: 12px;
    padding: 3px 7px;
    text-align: center;
  }
  input[type=number]:focus { outline: none; border-color: #383838; }

  select {
    background: #0d0d0d;
    border: 1px solid #1e1e1e;
    border-radius: 4px;
    color: #777;
    font-family: inherit;
    font-size: 11px;
    padding: 3px 6px;
    cursor: pointer;
  }
  select:disabled { color: #2a2a2a; cursor: default; }

  input[type=range] {
    width: 100%;
    accent-color: #444;
    margin-top: 2px;
  }
  input[type=range]:disabled { opacity: 0.2; }

  button.full {
    flex: 1;
    padding: 7px;
    background: none;
    border: 1px solid #1e1e1e;
    border-radius: 4px;
    color: #4a4a4a;
    font-family: inherit;
    font-size: 11px;
    cursor: pointer;
    width: 100%;
  }
  button.full:hover { color: #aaa; border-color: #333; }
  .section > button.full { margin-top: 10px; }

  .dim { opacity: 0.35; pointer-events: none; }
  .dim-label {
    font-size: 10px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: #444;
    margin-bottom: 10px;
  }

  /* Patterns tab */
  .pattern-list { padding: 8px 0; }

  .group-label {
    padding: 8px 14px 4px;
    font-size: 10px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: #333;
  }

  .pattern-item {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
    background: none;
    border: none;
    padding: 6px 14px;
    cursor: pointer;
    text-align: left;
    border-radius: 0;
    gap: 2px;
  }
  .pattern-item:hover { background: #161616; }

  .pattern-name {
    color: #777;
    font-size: 12px;
    font-family: inherit;
  }
  .pattern-item:hover .pattern-name { color: #bbb; }

  .pattern-meta {
    color: #333;
    font-size: 10px;
    font-family: inherit;
  }

  .rle-label {
    font-size: 10px;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: #3a3a3a;
    margin-bottom: 8px;
  }

  textarea {
    width: 100%;
    background: #0d0d0d;
    border: 1px solid #1e1e1e;
    border-radius: 4px;
    color: #777;
    font-family: inherit;
    font-size: 11px;
    padding: 6px 8px;
    resize: vertical;
    line-height: 1.4;
  }
  textarea:focus { outline: none; border-color: #383838; }
  textarea::placeholder { color: #2a2a2a; }
</style>
