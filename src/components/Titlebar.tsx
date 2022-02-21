import { getAll } from '@tauri-apps/api/window'

export default function Titlebar() {
  function minimize() {
    getAll().forEach(async window => await window.minimize())
  }

  async function fullscreen() {
    for (const window of getAll()) {
      console.log(window);
      await window.toggleMaximize()
    }
  }

  function close() {
    getAll().forEach(async window => await window.close())
  }

  return (
    <div data-tauri-drag-region className='w-full h-[30px]'>
      <div className="w-32 grid grid-cols-3 ml-auto items-center h-full">
        <button className='flex justify-center items-center w-full h-full hover:bg-white/10 text-white' onClick={() => minimize()}><svg className='w-3 h-3' fill='currentColor' x="0px" y="0px" viewBox="0 0 10.2 1"><rect x="0" y="50%" width="10.2" height="1" /></svg></button>
        <button className='flex justify-center items-center w-full h-full hover:bg-white/10' onClick={() => fullscreen()}><svg  fill='currentColor' className='w-3 h-3' viewBox="0 0 10 10"><path d="M0,0v10h10V0H0z M9,9H1V1h8V9z" /></svg></button>
        <button className='flex justify-center items-center w-full h-full hover:bg-white/10' onClick={() => close()}>
          <svg  fill='currentColor' className='w-3 h-3' viewBox="0 0 10 10"><polygon points="10.2,0.7 9.5,0 5.1,4.4 0.7,0 0,0.7 4.4,5.1 0,9.5 0.7,10.2 5.1,5.8 9.5,10.2 10.2,9.5 5.8,5.1" /></svg>
        </button>
      </div>
    </div>
  )
}
