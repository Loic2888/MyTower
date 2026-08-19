<script lang="ts">
  export type ToolIconName = 'kolibri' | 'gps' | 'jeux' | 'messagerie' | 'bibliotheque'

  let { name }: { name: ToolIconName } = $props()
</script>

<svg
  viewBox="0 0 24 24"
  class="h-5 w-5"
  fill="none"
  stroke="currentColor"
  stroke-width="1.5"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  {#if name === 'kolibri'}
    <polygon points="12,4 22,9 12,14 2,9" />
    <line x1="7" y1="11.5" x2="7" y2="17" />
    <circle cx="7" cy="17.6" r="1.1" fill="currentColor" stroke="none" />
  {:else if name === 'gps'}
    <circle cx="12" cy="9" r="4" />
    <polygon points="8,12 16,12 12,20" />
  {:else if name === 'jeux'}
    <circle cx="12" cy="6" r="2.3" />
    <polygon points="9,10.5 15,10.5 17,17 7,17" />
    <rect x="6" y="18" width="12" height="2.3" rx="1" />
  {:else if name === 'messagerie'}
    <rect x="7" y="2" width="10" height="20" rx="2" />
    <line x1="11" y1="18" x2="13" y2="18" />
  {:else if name === 'bibliotheque'}
    <rect x="6" y="8" width="12" height="3" rx="1" />
    <rect x="5" y="12" width="14" height="3" rx="1" />
    <rect x="4" y="16" width="16" height="3" rx="1" />
  {/if}
</svg>
