import { Navigate, Outlet } from 'react-router'
import { useStorageStore } from '@/store/useStorageStore'

/** 需登录 */
export const RequireAuth = () => {
  const token = useStorageStore.getState().token
  if (!token) {
    const redirectTo = encodeURIComponent(
      `${window.location.pathname}${window.location.search}`
    )
    return <Navigate to={`/login?redirect=${redirectTo}`} replace />
  }

  return <Outlet />
}

/** 仅游客（已登录访问登录页则回首页） */
export const GuestOnly = () => {
  const token = useStorageStore.getState().token
  if (token) {
    return <Navigate to="/" replace />
  }

  return <Outlet />
}
