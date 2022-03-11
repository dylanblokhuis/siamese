import { PauseIcon, PlayIcon, VolumeUpIcon } from "@heroicons/react/solid"
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from "@tauri-apps/api/window"
import { useStore } from "../state";
import { secondsIntoPretty } from "../utils/time";
import Seekbar from "./Seekbar";
import VolumeControls from "./VolumeControls";

export default function Controls() {
  // should probably split up controls to prevent constant rerendering
  const store = useStore();

  async function toggleFullscreen() {
    const isFullscreen = await appWindow.isFullscreen();

    await appWindow.setFullscreen(!isFullscreen)
  }

  return (
    <div className="flex flex-col mt-auto bg-white/5">
      <Seekbar />

      <div className="flex items-center justify-between px-4 pt-3 pb-4">
        <div className="flex items-center">
          {store.paused ? (
            <button type="button" onClick={() => invoke('toggle_playing')}>
              <PlayIcon className="w-8 h-8" />
            </button>
          ) : (
            <button type="button" onClick={() => invoke('toggle_playing')}>
              <PauseIcon className="w-8 h-8" />
            </button>
          )}

          <div className="ml-4">
            <VolumeControls />
          </div>

          <div className="ml-4">
            {secondsIntoPretty(store.time)} / {secondsIntoPretty(store.duration)}
          </div>
        </div>

        <div className="flex items-center">
          <button type="button" onClick={() => toggleFullscreen()}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              fill="none"
              stroke="currentColor"
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              className="feather feather-maximize"
              viewBox="0 0 24 24"
            >
              <path d="M8 3H5a2 2 0 00-2 2v3m18 0V5a2 2 0 00-2-2h-3m0 18h3a2 2 0 002-2v-3M3 16v3a2 2 0 002 2h3"></path>
            </svg>
          </button>
        </div>
      </div>
    </div>

  )
}
