import { useEffect } from 'react'
import { useUserStore } from '@/store/useUserStore'

const UserManagePage = () => {
  const { users, fetchUsers } = useUserStore()

  useEffect(() => {
    fetchUsers()
  }, [fetchUsers])

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
