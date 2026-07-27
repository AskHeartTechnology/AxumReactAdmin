import { useEffect } from 'react'
import { useUserStore } from '@/store/useUserStore'
import { Button } from '@/components/ui/button'

const UserManagePage = () => {
  const { users, fetchUsers } = useUserStore()

  useEffect(() => {
    fetchUsers()
  }, [fetchUsers])

  const handleClick = () => {
    alert('按钮被点击了')
  }

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
      <Button onClick={handleClick}>点击</Button>
    </div>
  )
}

export default UserManagePage
