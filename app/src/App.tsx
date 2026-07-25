import { useState } from 'react'
import Login from '@/components/auth/Login'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <button
        type="button"
        className="counter"
        onClick={() => setCount(count => count + 1)}
      >
        Count is {count}
      </button>

      <Login />
    </>
  )
}

export default App
