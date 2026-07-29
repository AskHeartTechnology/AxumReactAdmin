import type { RouteObject } from 'react-router'
import { lazyLoad } from './lazy'

/**
 * 只在这里维护路由表：
 * - page: 组件在 src/pages 下的相对地址
 */
export const appRouteConfig: Array<{
  path?: string
  index?: boolean
  page: string
}> = [
  { index: true, page: 'home1' },
  { path: 'user', page: 'user' },
]

export function createAppRoutes(): RouteObject[] {
  return appRouteConfig.map(item => {
    if (item.index) {
      return {
        index: true,
        element: lazyLoad(item.page),
      }
    }

    return {
      path: item.path,
      element: lazyLoad(item.page),
    }
  })
}

export function createLoginRoute(): RouteObject {
  return {
    path: '/login',
    element: lazyLoad('auth/login'),
  }
}
