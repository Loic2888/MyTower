import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export type ChatMessage = {
  role: 'user' | 'assistant'
  content: string
}

const GREETINGS = [
  "Bonjour ! Contente de te retrouver — dis-moi si je peux t'aider.",
  'Salut ! Je suis là, prête à t’accompagner sur ce que tu veux faire aujourd’hui.',
  "Bonjour ! N'hésite pas si tu as besoin d'un coup de main.",
]

function pickGreeting(): string {
  return GREETINGS[Math.floor(Math.random() * GREETINGS.length)]
}

export const chat = $state({
  messages: [{ role: 'assistant', content: pickGreeting() }] as ChatMessage[],
  sending: false,
  errorMessage: null as string | null,
  thinkEnabled: false,
})

// Écouté une seule fois pour toute la durée de vie de l'app : que le flux
// vienne d'un message envoyé par l'utilisateur ou d'un check-in déclenché
// par l'app, il complète toujours le dernier message assistant.
listen<string>('chat:chunk', (event) => {
  const last = chat.messages[chat.messages.length - 1]
  if (last?.role === 'assistant') {
    last.content += event.payload
  }
})

export async function sendMessage(draft: string) {
  const content = draft.trim()
  if (!content || chat.sending) return

  chat.errorMessage = null
  chat.messages.push({ role: 'user', content })
  chat.messages.push({ role: 'assistant', content: '' })
  chat.sending = true

  // On n'envoie pas le placeholder assistant vide qu'on vient d'ajouter.
  const history = chat.messages.slice(0, -1)

  try {
    await invoke('send_chat_message', { history, think: chat.thinkEnabled })
  } catch (err) {
    chat.errorMessage = typeof err === 'string' ? err : "Erreur lors de l'appel à Ollama."
  } finally {
    chat.sending = false
  }
}

/** Déclenché par ToolsPanel quand un outil se ferme (pas par l'utilisateur). */
export async function checkInOnTool(label: string) {
  if (chat.sending) return

  chat.errorMessage = null
  chat.messages.push({ role: 'assistant', content: '' })
  chat.sending = true

  try {
    await invoke('trigger_checkin', { toolLabel: label })
  } catch (err) {
    chat.errorMessage = typeof err === 'string' ? err : "Erreur lors de l'appel à Ollama."
  } finally {
    chat.sending = false
  }
}
