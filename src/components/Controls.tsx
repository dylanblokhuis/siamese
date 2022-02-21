import { PauseIcon, PlayIcon } from "@heroicons/react/solid"
import { emit } from "@tauri-apps/api/event"

export default function Controls() {
  return (
    <div className="h-16 flex items-center px-4 mt-auto border-t-blue-500 border-t-2">
      <button type="button" onClick={() => emit('start')}>
        <PlayIcon className="w-8 h-8" />
      </button>
      <button type="button" onClick={() => emit('pause')}>
        <PauseIcon className="w-8 h-8" />
      </button>
    </div>
  )
}
