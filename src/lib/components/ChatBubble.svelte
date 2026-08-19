<script lang="ts">
  let {
    role,
    content,
  }: {
    role: 'user' | 'assistant'
    content: string
  } = $props()
</script>

<div
  class="max-w-[80%] rounded-2xl px-4 py-2.5 text-sm whitespace-pre-wrap {role === 'user'
    ? 'self-end bg-gradient-to-br from-violet-600 to-blue-600 text-white shadow-lg shadow-violet-950/40'
    : 'self-start border border-white/10 bg-white/5 text-slate-200 backdrop-blur'}"
>
  {content || '…'}
</div>
