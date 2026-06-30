<script>
  import { createEventDispatcher } from 'svelte'

  export let value = 'life'

  const dispatch = createEventDispatcher()
  let open = false

  const groups = [
    { label: '2D', items: [
      { id: 'life',     label: 'Game of Life' },
      { id: 'highlife', label: 'HighLife',       disabled: true },
      { id: 'brian',    label: "Brian's Brain",  disabled: true },
    ]},
    { label: '1D', items: [
      { id: 'wolfram',  label: 'Wolfram Elementary', disabled: true },
    ]},
    { label: '3D', items: [
      { id: 'life3d',   label: 'Game of Life 3D', disabled: true },
    ]},
    { label: 'Continuous', items: [
      { id: 'smoothlife', label: 'SmoothLife', disabled: true },
    ]},
  ]

  $: label = groups.flatMap(g => g.items).find(i => i.id === value)?.label ?? value

  function pick(id) {
    dispatch('change', id)
    open = false
  }
</script>

<svelte:window on:click={() => open = false} />

<div class="ca-dropdown" on:click|stopPropagation>
  <button class="trigger" on:click={() => open = !open}>
    Menu<span class="caret">▾</span>
  </button>

  {#if open}
    <div class="menu">
      {#each groups as group}
        <div class="group-label">{group.label}</div>
        {#each group.items as item}
          <button
            class="item"
            class:active={item.id === value}
            class:muted={item.disabled}
            disabled={item.disabled}
            on:click={() => pick(item.id)}
          >{item.label}</button>
        {/each}
      {/each}
    </div>
  {/if}
</div>

<style>
  .ca-dropdown { position: relative; }

  .trigger {
    background: none;
    border: none;
    color: #888;
    font-family: inherit;
    font-size: 12px;
    letter-spacing: 0.04em;
    cursor: pointer;
    padding: 4px 0;
    display: flex;
    align-items: center;
    gap: 5px;
  }
  .trigger:hover { color: #ccc; }

  .caret { font-size: 8px; color: #555; }

  .menu {
    position: absolute;
    top: calc(100% + 8px);
    left: 0;
    background: #111;
    border: 1px solid #222;
    border-radius: 6px;
    padding: 6px;
    min-width: 190px;
    z-index: 100;
  }

  .group-label {
    color: #333;
    font-size: 10px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    padding: 6px 8px 3px;
  }

  .item {
    display: block;
    width: 100%;
    background: none;
    border: none;
    color: #777;
    font-family: inherit;
    font-size: 12px;
    text-align: left;
    padding: 5px 8px;
    border-radius: 4px;
    cursor: pointer;
  }
  .item:hover:not(:disabled) { background: #1a1a1a; color: #ccc; }
  .item.active { color: #ddd; }
  .item.muted { color: #2e2e2e; cursor: default; }
</style>
