import { invoke } from '@tauri-apps/api/tauri'
import Titlebar from './components/Titlebar'
import Controls from './components/Controls'

function App() {
  return (
    <div className='h-full flex flex-col'>
      <Titlebar />
      <Controls />
    </div>
  )
}

export default App
