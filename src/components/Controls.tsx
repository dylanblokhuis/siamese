import { PauseIcon, PlayIcon, StopIcon } from "@heroicons/react/solid"
import { Event } from "@tauri-apps/api/event";
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from "@tauri-apps/api/window"
import { useEffect, useState } from "react";
import { secondsIntoPretty } from "../utils/time";

interface OnPlayerUpdatePayload {
  time: number,
  duration: number,
  visible: boolean
}

export default function Controls() {
  const [playerDetails, setPlayerDetails] = useState<OnPlayerUpdatePayload>({
    time: 0,
    duration: 0,
    visible: false
  });

  async function toggleFullscreen() {
    const isFullscreen = await appWindow.isFullscreen();

    await appWindow.setFullscreen(!isFullscreen)
  }

  useEffect(() => {
    appWindow.listen('on-player-update', (event: Event<OnPlayerUpdatePayload>) => {
      setPlayerDetails(event.payload);
    })

    invoke('start').then(() => console.log("invoked start"));
  }, []);

  console.log(secondsIntoPretty(playerDetails.duration));
  
  return (
    <div className="h-16 flex items-center justify-between px-4 mt-auto border-t-blue-500 border-t-2">
      <div className="flex items-center">
        <button type="button" onClick={() => invoke('play')}>
          <PlayIcon className="w-8 h-8" />
        </button>
        <button type="button" onClick={() => invoke('pause')}>
          <PauseIcon className="w-8 h-8" />
        </button>
        <button type="button" onClick={() => invoke('stop')}>
          <StopIcon className="w-8 h-8" />
        </button>

        <div className="ml-4">
          {secondsIntoPretty(playerDetails.time)} / {secondsIntoPretty(playerDetails.duration)}
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
  )
}
