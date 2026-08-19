<script lang="ts">
  import type { Snippet } from 'svelte'

  let {
    label,
    active = false,
    onclick,
    children,
  }: {
    label: string
    active?: boolean
    onclick?: () => void
    children: Snippet
  } = $props()
</script>

<button
  type="button"
  {onclick}
  aria-label={label}
  class="group relative flex h-11 w-11 shrink-0 items-center justify-center rounded-xl text-slate-500 transition duration-300 hover:bg-white/5 hover:text-violet-300 hover:shadow-[0_0_20px_-4px_rgba(139,92,246,0.7)] {active
    ? 'bg-white/5 text-violet-300 shadow-[0_0_20px_-4px_rgba(139,92,246,0.7)]'
    : ''}"
>
  {@render children()}

  <span
    class="pointer-events-none absolute left-full ml-3 whitespace-nowrap rounded-lg border border-white/10 bg-slate-900/90 px-2.5 py-1 text-xs font-medium text-slate-200 opacity-0 shadow-lg backdrop-blur-xl transition-opacity duration-100 group-hover:opacity-100"
  >
    {label}
  </span>
</button>
