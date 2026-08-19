<script lang="ts">
  import GlassPanel from './components/GlassPanel.svelte'
  import PanelHeading from './components/PanelHeading.svelte'
  import PillToggle from './components/PillToggle.svelte'
  import ChatBubble from './components/ChatBubble.svelte'
  import GradientButton from './components/GradientButton.svelte'
  import { chat, sendMessage } from './chatStore.svelte'

  let draft = $state('')

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault()
    if (!draft.trim() || chat.sending) return
    const value = draft
    draft = ''
    sendMessage(value)
  }
</script>

<GlassPanel glow="blue">
  <div class="flex h-full flex-col p-5">
    <div class="flex items-center justify-between">
      <PanelHeading>Assistant</PanelHeading>
      <PillToggle
        active={chat.thinkEnabled}
        onclick={() => (chat.thinkEnabled = !chat.thinkEnabled)}
        title="Quand activé, le modèle réfléchit avant de répondre (beaucoup plus lent)."
        activeLabel="Réflexion activée"
        inactiveLabel="Réflexion désactivée"
        activeColor="amber"
      />
    </div>

    <div class="mt-4 flex flex-1 flex-col gap-3 overflow-y-auto pr-1">
      {#each chat.messages as message}
        <ChatBubble role={message.role} content={message.content} />
      {/each}
    </div>

    {#if chat.errorMessage}
      <p class="mt-2 text-sm text-red-400">{chat.errorMessage}</p>
    {/if}

    <form
      class="mt-4 flex items-center gap-2 rounded-xl border border-white/10 bg-slate-950/50 p-1.5 transition-colors duration-300 focus-within:border-violet-500/40 focus-within:shadow-[0_0_20px_-6px_rgba(139,92,246,0.6)]"
      onsubmit={handleSubmit}
    >
      <input
        type="text"
        bind:value={draft}
        disabled={chat.sending}
        placeholder="Écris ton message…"
        class="flex-1 bg-transparent px-3 py-2 text-sm text-slate-100 placeholder-slate-600 focus:outline-none disabled:opacity-50"
      />
      <GradientButton type="submit" disabled={chat.sending || !draft.trim()}>Envoyer</GradientButton>
    </form>
  </div>
</GlassPanel>
