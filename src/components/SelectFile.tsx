import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog"

import { useStore } from "../state"

export default function SelectFile() {
  const store = useStore();
  if (store.visible) return null;

  async function select() {
    const filePath = await open({
      filters: [{
        name: "General",
        extensions: [
          "mkv",
          "mp4",
          "webm",
          "flv",
          "flv",
          "vob",
          "ogv",
          "ogg",
          "drc",
          "avi",
          "wmv",
          "mpeg",
          "flv"
        ]
      }]
    });

    invoke('start', {
      filePath
    })
  }

  return (
    <div className="fixed left-0 top-0 bottom-0 right-0 m-auto w-64 h-16 flex items-center justify-center p-4 rounded-3xl">
      <button className="bg-blue-500 px-5 py-2 rounded-lg" onClick={select} type="button">
        Select a file
      </button>
    </div>
  )
}
