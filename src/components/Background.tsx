import clsx from 'clsx'
import { useStore } from '../state'

export default function Background() {
  const store = useStore();
  if (store.visible) return null

  return (
    <div className={clsx("absolute w-full h-full left-0 top-0 bg-black -z-10")} />
  )
}
