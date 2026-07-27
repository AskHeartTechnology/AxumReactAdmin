import { create } from 'zustand'
import { createJSONStorage, persist } from 'zustand/middleware'

interface StorageStates {
  token: string | null
}

interface StorageActions {
  setToken: (token: string | null) => void
}

type StorageStore = StorageStates & StorageActions

export const useStorageStore = create<StorageStore>()(
  persist(
    set => ({
      token: null,

      setToken: token => set({ token }),
    }),
    {
      name: 'app-storage-store',
      storage: createJSONStorage(() => localStorage),
    }
  )
)
