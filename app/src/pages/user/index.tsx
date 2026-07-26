import type { IUser } from '@/api/types/user'
import { getUsers } from '@/api/user'
import { ResponseCode } from '@/utils/request/types'
import { useEffect, useState } from 'react'

const UserManagePage = () => {
  const [users, setUsers] = useState<IUser[]>([])

  const fetchUsers = async () => {
    const { code, data } = await getUsers()
    if (code === ResponseCode.SUCCESS) {
      setUsers(data)
    } else {
      setUsers([])
    }
  }

  useEffect(() => {
    fetchUsers()
  }, [])

  return (
    <div className="user-manage-page">
      <h1>用户管理</h1>
      <ul>
        {users.map(item => {
          return (
            <li key={item.id}>
              {item.name}({item.email})
            </li>
          )
        })}
      </ul>
    </div>
  )
}

export default UserManagePage
