import create from 'zustand'
import { appWindow } from "@tauri-apps/api/window"
import { Event } from '@tauri-apps/api/event';

interface AppStore {
  time: number
  duration: number
  visible: boolean
  paused: boolean
  volume: number
}
export const useStore = create<AppStore>(set => ({
  time: 0,
  duration: 0,
  visible: false,
  paused: true,
  volume: 100
}))

interface OnPlayerUpdatePayload {
  time: number,
  duration: number,
  visible: boolean,
  paused: boolean,
  volume: number
}
appWindow.listen('on-player-update', (event: Event<OnPlayerUpdatePayload>) => {
  useStore.setState({
    duration: event.payload.duration,
    time: event.payload.time,
    visible: event.payload.visible,
    paused: event.payload.paused,
    volume: event.payload.volume
  })

  console.log(event.payload);
})