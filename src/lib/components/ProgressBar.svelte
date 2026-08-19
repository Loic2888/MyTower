<script lang="ts">
  let { percent }: { percent: number } = $props()

  const clamped = $derived(Math.min(100, Math.max(0, percent)))
</script>

<div class="h-1.5 w-full overflow-hidden rounded-full bg-white/10">
  <div
    class="h-full rounded-full bg-gradient-to-r from-violet-500 to-blue-500 transition-[width] duration-300"
    style="width: {clamped}%"
  ></div>
</div>
