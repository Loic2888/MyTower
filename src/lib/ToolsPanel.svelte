<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { openUrl } from '@tauri-apps/plugin-opener'
  import GlassPanel from './components/GlassPanel.svelte'
  import PanelHeading from './components/PanelHeading.svelte'
  import IconRailButton from './components/IconRailButton.svelte'
  import GradientButton from './components/GradientButton.svelte'
  import ProgressBar from './components/ProgressBar.svelte'
  import ToolIcon, { type ToolIconName } from './components/ToolIcon.svelte'
  import { checkInOnTool } from './chatStore.svelte'
  import { wizard, downloadEntry, type ZimEntry } from './wizardStore.svelte'
  import wikipediaCollection from '../../collections/wikipedia.json'
  import kolibriCollection from '../../collections/kolibri.json'
  import mapsCollection from '../../collections/maps.json'

  type Tool = { id: ToolIconName; label: string }

  // Étape 3 du séquencement : chaque outil s'intègre ici un par un, sans
  // tool-calling à ce stade. Rien n'est fonctionnel pour l'instant — la
  // liste sert juste à donner sa place à chacun dans l'UI.
  const tools: Tool[] = [
    { id: 'kolibri', label: 'Académie' },
    { id: 'gps', label: 'GPS' },
    { id: 'jeux', label: 'Salle de jeux' },
    { id: 'messagerie', label: 'Messagerie' },
    { id: 'bibliotheque', label: 'Contenus' },
  ]

  // En dessous de cette durée d'ouverture, on considère que c'était juste un
  // clic curieux (rien n'est fonctionnel de toute façon) — pas la peine de
  // solliciter Ollama pour un check-in dans ce cas.
  const CHECKIN_MIN_DURATION_MS = 4000

  const kolibriUrl = import.meta.env.VITE_KOLIBRI_URL ?? 'http://localhost:8080'

  let selected = $state<Tool | null>(null)
  let openedAt = 0

  function logEvent(kind: 'tool_open' | 'tool_close', label: string) {
    // Le log de session est un "nice to have" : une erreur ici ne doit
    // jamais empêcher d'utiliser l'outil.
    invoke('log_session_event', { kind, label }).catch(() => {})
  }

  function close(tool: Tool) {
    logEvent('tool_close', tool.label)
    if (Date.now() - openedAt >= CHECKIN_MIN_DURATION_MS) {
      checkInOnTool(tool.label)
    }
  }

  // Kolibri refuse de s'afficher en iframe (X-Frame-Options: DENY) et,
  // pour la perf, on a choisi le navigateur système plutôt qu'une seconde
  // fenêtre Tauri (même moteur WebKitGTK que la fenêtre principale, déjà
  // identifié comme coûteux dans cet environnement). On ne sait donc pas
  // quand l'utilisateur a fini — pas de suivi d'ouverture/fermeture ni de
  // check-in pour cet outil, juste le log d'ouverture.
  function openKolibri(tool: Tool) {
    logEvent('tool_open', tool.label)
    openUrl(kolibriUrl).catch(() => {})
    selected = tool
  }

  function toggle(tool: Tool) {
    if (tool.id === 'kolibri') {
      openKolibri(tool)
      return
    }

    if (selected?.id === tool.id) {
      close(tool)
      selected = null
      return
    }

    if (selected && selected.id !== 'kolibri') {
      close(selected)
    }

    logEvent('tool_open', tool.label)
    openedAt = Date.now()
    selected = tool
  }

  function progressPercent(entry: ZimEntry): number {
    const progress = wizard.progress[entry.id]
    if (!progress || !progress.total) return 0
    return (progress.downloaded / progress.total) * 100
  }
</script>

<GlassPanel glow="violet">
  <div class="flex h-full">
    <nav class="flex w-16 flex-col items-center gap-2 border-r border-white/10 py-4">
      {#each tools as tool (tool.id)}
        <IconRailButton label={tool.label} active={selected?.id === tool.id} onclick={() => toggle(tool)}>
          <ToolIcon name={tool.id} />
        </IconRailButton>
      {/each}
    </nav>

    <div class="flex flex-1 flex-col overflow-hidden p-5">
      <PanelHeading>Outils</PanelHeading>

      <div class="mt-4 flex flex-1 flex-col overflow-hidden">
        {#if selected?.id === 'bibliotheque'}
          <div class="flex flex-1 flex-col gap-5 overflow-y-auto pr-1">
            <div>
              <h3 class="text-sm font-medium text-slate-300">{wikipediaCollection.category}</h3>
              {#if wikipediaCollection.note}
                <p class="mt-1 text-xs text-slate-500">{wikipediaCollection.note}</p>
              {/if}
              <div class="mt-3 flex flex-col gap-3">
                {#each wikipediaCollection.entries as entry (entry.id)}
                  <div class="rounded-lg border border-white/10 bg-white/5 p-3">
                    <div class="flex items-center justify-between gap-3">
                      <div>
                        <p class="text-sm font-medium text-slate-200">{entry.title}</p>
                        <p class="text-xs text-slate-500">{entry.description} · {entry.sizeLabel}</p>
                      </div>
                      <GradientButton
                        disabled={wizard.activeId !== null}
                        onclick={() => downloadEntry(entry)}
                      >
                        {wizard.activeId === entry.id ? '…' : 'Télécharger'}
                      </GradientButton>
                    </div>
                    {#if wizard.progress[entry.id]}
                      <div class="mt-2">
                        <ProgressBar percent={progressPercent(entry)} />
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>

            <div>
              <h3 class="text-sm font-medium text-slate-300">{kolibriCollection.category}</h3>
              <p class="mt-1 text-xs text-slate-500">{kolibriCollection.note}</p>
            </div>

            <div>
              <h3 class="text-sm font-medium text-slate-300">{mapsCollection.category}</h3>
              <p class="mt-1 text-xs text-slate-500">{mapsCollection.note}</p>
            </div>

            {#if wizard.errorMessage}
              <p class="text-sm text-red-400">{wizard.errorMessage}</p>
            {/if}
          </div>
        {:else if selected?.id === 'kolibri'}
          <div class="flex flex-1 flex-col items-center justify-center gap-2 text-center">
            <p class="text-base font-medium text-slate-200">Académie</p>
            <p class="max-w-[16rem] text-sm text-slate-500">
              Ouverte dans ton navigateur — Kolibri bloque son affichage direct dans l'app.
            </p>
          </div>
        {:else if selected}
          <div class="flex flex-1 flex-col items-center justify-center gap-2 text-center">
            <p class="text-base font-medium text-slate-200">{selected.label}</p>
            <p class="max-w-[16rem] text-sm text-slate-500">
              Bientôt disponible — intégration prévue à l'étape 3 du séquencement.
            </p>
          </div>
        {:else}
          <div class="flex flex-1 items-center justify-center text-center">
            <p class="text-sm text-slate-600">Sélectionne un outil dans le rail à gauche.</p>
          </div>
        {/if}
      </div>
    </div>
  </div>
</GlassPanel>
