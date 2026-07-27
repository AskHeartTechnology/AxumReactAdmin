import { get, post, put, del } from '@/utils/request'
import type { IUser } from './types/user'

export const getUsers = (params?: { pageNum: number; pageSize: number }) => {
  return get<IUser[], typeof params>(`/users/list`, params)
}

export const getUser = (id: string) => {
  return get<IUser, undefined>(`/users/detail/${id}`)
}

export const createUser = () => {
  return post(`/users/create`)
}

export const updateUser = (id: string) => {
  return put(`/users/update/${id}`)
}

export const deleteUser = (id: string) => {
  return del(`/users/delete/${id}`)
}
