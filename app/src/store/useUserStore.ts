import { create } from 'zustand'
import type { IUser } from '@/api/types/user'
import { getUser, getUsers } from '@/api/user'
import { ResponseCode } from '@/utils/request/types'

interface UserStates {
  users: IUser[]
  user: IUser | null
}

interface UserActions {
  fetchUser: (id: string) => void
  fetchUsers: () => void
}

type UserStore = UserStates & UserActions

export const useUserStore = create<UserStore>()(set => ({
  user: null,
  users: [],
  fetchUser: async id => {
    const { code, data } = await getUser(id)
    if (code === ResponseCode.SUCCESS) {
      set({ user: data ?? null })
    } else {
      set({ user: null })
    }
  },
  fetchUsers: async () => {
    const { code, data } = await getUsers()
    if (code === ResponseCode.SUCCESS) {
      set({ users: data.length ? data : [] })
    } else {
      set({ users: [] })
    }
  },
}))
