<script lang="ts">
  type Color = 'violet' | 'amber'

  let {
    active,
    onclick,
    title = '',
    activeLabel,
    inactiveLabel,
    activeColor = 'violet',
  }: {
    active: boolean
    onclick: () => void
    title?: string
    activeLabel: string
    inactiveLabel: string
    activeColor?: Color
  } = $props()

  const activeClasses: Record<Color, string> = {
    violet:
      'border-violet-400/30 bg-violet-500/15 text-violet-300 shadow-[0_0_16px_-4px_rgba(139,92,246,0.6)]',
    amber:
      'border-amber-400/30 bg-amber-500/15 text-amber-300 shadow-[0_0_16px_-4px_rgba(245,158,11,0.6)]',
  }
</script>

<button
  type="button"
  {onclick}
  {title}
  class="rounded-full border px-3 py-1 text-xs font-medium transition duration-300 {active
    ? activeClasses[activeColor]
    : 'border-white/10 bg-white/5 text-slate-500 hover:text-slate-300'}"
>
  {active ? activeLabel : inactiveLabel}
</button>
