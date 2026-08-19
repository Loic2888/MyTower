<script lang="ts">
  import type { Snippet } from 'svelte'

  let { children }: { children: Snippet } = $props()
</script>

<h2 class="text-sm font-semibold tracking-wide text-slate-400 uppercase">
  {@render children()}
</h2>
