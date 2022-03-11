import { invoke } from "@tauri-apps/api/tauri";
import React, { useRef, useState } from "react";
import { useStore } from "../state"
import { secondsIntoPretty } from "../utils/time";

interface TooltipProps {
  label: string
  left: string
}

export default function Seekbar() {
  const store = useStore();
  const [isTooltipOpen, setIsTooltipOpen] = useState(false);
  const [tooltipProps, setTooltipProps] = useState<TooltipProps | null>()
  const ref = useRef<HTMLDivElement>(null);

  function getTimeFromSeekbarMouseMovement(event: React.MouseEvent<HTMLDivElement>) {
    const el = ref.current;
    if (!el) return

    const percentage = event.clientX / el.clientWidth;
    const time =  store.duration * percentage;

    setTooltipProps({
      label: secondsIntoPretty(time),
      left: `${percentage * 100}%`
    })
  }

  function onSeek(event: React.MouseEvent<HTMLDivElement>) {
    const el = ref.current;
    if (!el) return

    const percentage = event.clientX / el.clientWidth;
    const time =  store.duration * percentage;

    invoke("seek", {
      timePos: Number(time.toFixed(0))
    })
  }

  const durationInPercentage = store.duration === 0 ? 0 : (store.time / store.duration) * 100

  return (
    <div ref={ref} onClick={onSeek} onMouseEnter={() => setIsTooltipOpen(true)} onMouseLeave={() => setIsTooltipOpen(false)} role="button" className="w-full h-2 bg-white/10 relative" onMouseMove={getTimeFromSeekbarMouseMovement}>
      {isTooltipOpen && tooltipProps && (
        <div className="absolute bottom-[150%] h-6 px-1 rounded bg-red-500 leading-none flex items-center text-sm" style={{ left: tooltipProps.left}}>
          {tooltipProps.label}
        </div>
      )}
      
      <div className="w-full h-full bg-red-500 transition-all" style={{ width: `${durationInPercentage}%` }} />
    </div>
  )
}
