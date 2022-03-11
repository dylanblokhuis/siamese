import Titlebar from './components/Titlebar'
import Controls from './components/Controls'
import Background from './components/Background'
import SelectFile from './components/SelectFile'

function App() {
  return (
    <div className='h-full flex flex-col relative select-none'>
      <Background />
      <Titlebar />
      <SelectFile />
      <Controls />
    </div>
  )
}

export default App
