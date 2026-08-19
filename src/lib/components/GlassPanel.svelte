<script lang="ts">
  import type { Snippet } from 'svelte'

  type Glow = 'violet' | 'blue'

  let {
    glow = 'violet',
    class: className = '',
    children,
  }: {
    glow?: Glow
    class?: string
    children: Snippet
  } = $props()

  const glowShadow: Record<Glow, string> = {
    violet: 'shadow-violet-950/40',
    blue: 'shadow-blue-950/40',
  }
</script>

<div
  class="h-full w-full overflow-hidden rounded-2xl border border-white/10 bg-slate-900/60 shadow-2xl backdrop-blur-xl {glowShadow[
    glow
  ]} {className}"
>
  {@render children()}
</div>
