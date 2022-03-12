import { invoke } from "@tauri-apps/api/tauri";
import clsx from "clsx";
import React, { useRef, useState } from "react";
import { getTrackBackground, Range } from "react-range";
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
  const durationInPercentage = store.duration === 0 ? 0 : (store.time / store.duration) * 100
  const [seekbarFill, setSeekbarFill] = useState(durationInPercentage)
  const ref = useRef<HTMLDivElement>(null);

  const STEP = 1;
  const MIN = 0;
  const MAX = store.duration;

  function getTimeFromSeekbarMouseMovement(event: React.MouseEvent<HTMLDivElement>) {
    const el = ref.current;
    if (!el) return

    const percentage = event.clientX / el.clientWidth;
    const time = store.duration * percentage;

    setTooltipProps({
      label: secondsIntoPretty(time),
      left: `${percentage * 100}%`
    })
  }

  return (
    <div
      className="flex justify-center flex-wrap w-full relative"
    >
      {isTooltipOpen && tooltipProps && (
        <div className="absolute bottom-[150%] h-6 px-1 rounded bg-red-500 leading-none flex items-center text-sm" style={{ left: tooltipProps.left }}>
          {tooltipProps.label}
        </div>
      )}

      <Range
        values={[seekbarFill]}
        step={STEP}
        min={MIN}
        max={MAX}
        onChange={async ([value]) => {
          if (!isTooltipOpen) {
            setIsTooltipOpen(true)
          }

          setSeekbarFill(value)
          await invoke("seek", {
            timePos: value
          })
        }}
        onFinalChange={async () => {
          await invoke("seek", {
            timePos: seekbarFill
          })

          setIsTooltipOpen(false)
        }}
        renderTrack={({ props, children }) => (
          <div
            onMouseMove={getTimeFromSeekbarMouseMovement}
            onMouseEnter={() => setIsTooltipOpen(true)}
            onMouseLeave={() => setIsTooltipOpen(false)}
            onMouseDown={props.onMouseDown}
            onTouchStart={props.onTouchStart}
            ref={ref}
            style={{
              ...props.style,
              height: '10px',
              display: 'flex',
              width: '100%'
            }}
          >
            <div
              ref={props.ref}
              className="h-2 w-full self-center"
              style={{
                background: getTrackBackground({
                  values: [seekbarFill],
                  colors: ['rgb(239 68 68)', '#cccccc21'],
                  min: MIN,
                  max: MAX
                }),
              }}
            >
              {children}
            </div>
          </div>
        )}
        renderThumb={({ props, isDragged }) => (
          <div
            {...props}
            className={clsx("w-4 h-4 rounded-full outline-none", isDragged ? "bg-gray-300" : "bg-white")}
            style={{
              ...props.style,
            }}
          />
        )}
      />
    </div>
  )
}
