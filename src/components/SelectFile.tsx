import { invoke } from "@tauri-apps/api/tauri";
import { useStore } from "../state"

export default function SelectFile() {
  const store = useStore();
  if (store.visible) return null;

  return (
    <div className="fixed left-0 top-0 bottom-0 right-0 m-auto w-32 h-32 bg-slate-400 p-4 rounded-3xl">
      <button onClick={() => invoke('start')} type="button">
        Start!
      </button>
    </div>
  )
}
