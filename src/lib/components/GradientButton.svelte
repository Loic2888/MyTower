<script lang="ts">
  import type { Snippet } from 'svelte'

  let {
    type = 'button',
    disabled = false,
    onclick,
    children,
  }: {
    type?: 'button' | 'submit'
    disabled?: boolean
    onclick?: () => void
    children: Snippet
  } = $props()
</script>

<button
  {type}
  {disabled}
  {onclick}
  class="rounded-lg bg-gradient-to-br from-violet-600 to-blue-600 px-4 py-2 text-sm font-medium text-white shadow-lg shadow-violet-950/40 transition duration-300 hover:shadow-[0_0_20px_-4px_rgba(139,92,246,0.8)] disabled:cursor-not-allowed disabled:opacity-40 disabled:shadow-none"
>
  {@render children()}
</button>
