import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export type ZimEntry = {
  id: string
  title: string
  description: string
  url: string
  filename: string
  sizeLabel: string
}

type Progress = { downloaded: number; total: number | null }

export const wizard = $state({
  // Un seul téléchargement à la fois — évite de saturer la bande passante
  // et garde la gestion de progression simple.
  activeId: null as string | null,
  progress: {} as Record<string, Progress>,
  errorMessage: null as string | null,
})

listen<{ id: string; downloaded: number; total: number | null }>('wizard:progress', (event) => {
  wizard.progress[event.payload.id] = {
    downloaded: event.payload.downloaded,
    total: event.payload.total,
  }
})

export async function downloadEntry(entry: ZimEntry) {
  if (wizard.activeId) return

  wizard.errorMessage = null
  wizard.activeId = entry.id
  wizard.progress[entry.id] = { downloaded: 0, total: null }

  try {
    await invoke('download_zim', { id: entry.id, url: entry.url, filename: entry.filename })
  } catch (err) {
    wizard.errorMessage = typeof err === 'string' ? err : 'Erreur pendant le téléchargement.'
  } finally {
    wizard.activeId = null
  }
}
