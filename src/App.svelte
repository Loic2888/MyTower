<script lang="ts">
  import ToolsPanel from './lib/ToolsPanel.svelte'
  import ChatPanel from './lib/ChatPanel.svelte'
  import AmbientGlow from './lib/components/AmbientGlow.svelte'
</script>

<main class="relative flex h-screen w-screen gap-4 overflow-hidden bg-slate-950 p-4">
  <AmbientGlow />

  <div class="relative z-10 min-w-0 flex-[3]">
    <ToolsPanel />
  </div>
  <div class="relative z-10 min-w-0 flex-[2]">
    <ChatPanel />
  </div>
</main>
