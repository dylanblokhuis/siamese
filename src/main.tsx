import React from 'react'
import ReactDOM from 'react-dom'
import './index.css'
import App from './App'
import { invoke } from '@tauri-apps/api/tauri'

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
)

document.addEventListener("keydown", async (event) => {
  event.preventDefault();
  
  if (event.code === "Space") {
    await invoke('toggle_playing')
  }
});