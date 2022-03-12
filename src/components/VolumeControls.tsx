import { useEffect, useState } from "react";
import { Range, getTrackBackground } from "react-range";
import { invoke } from "@tauri-apps/api";
import clsx from "clsx";
import { VolumeUpIcon } from "@heroicons/react/solid";

import { useStore } from "../state";

const STEP = 1;
const MIN = 0;
const MAX = 100;

export default function VolumeControls() {
  const playerVolume = useStore(state => state.volume);
  const [isOpen, setIsOpen] = useState(false);
  const [volume, setVolume] = useState<number>(playerVolume)

  useEffect(() => {
    invoke('set_volume', {
      volume
    });
  }, [volume])

  return (
    <div className="flex items-center">
      <button onClick={() => setIsOpen(!isOpen)} type="button">
        <VolumeUpIcon className="w-6 h-6" />
      </button>

      {!isOpen && (
        <div
          className="flex justify-center flex-wrap w-24 mx-3"
        >
          <Range
            values={[volume]}
            step={STEP}
            min={MIN}
            max={MAX}
            onChange={([value]) => {
              setVolume(value)
            }}
            renderTrack={({ props, children }) => (
              <div
                onMouseDown={props.onMouseDown}
                onTouchStart={props.onTouchStart}
                style={{
                  ...props.style,
                  height: '30px',
                  display: 'flex',
                  width: '100%'
                }}
              >
                <div
                  ref={props.ref}
                  className="h-1 w-full rounded self-center"
                  style={{
                    background: getTrackBackground({
                      values: [volume],
                      colors: ['rgb(239 68 68)', '#ccc'],
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
                className={clsx("w-4 h-4 rounded-full", isDragged ? "bg-gray-300" : "bg-white")}
                style={{
                  ...props.style,
                }}
              />
            )}
          />
        </div>
      )}
    </div>
  )
}
