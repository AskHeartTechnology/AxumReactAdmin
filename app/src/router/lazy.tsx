import { lazy, Suspense, type ComponentType, type ReactNode } from 'react'
import NotFoundPage from '@/pages/auth/404'

type PageModule = { default: ComponentType }

/**
 * 预扫描页面模块（构建期生成映射表）
 * 这里只负责“能找到组件”，不负责生成 path
 */
const pageModules = import.meta.glob<PageModule>('../pages/**/*.{tsx,jsx}')

const withSuspense = (Comp: ComponentType): ReactNode => {
  return (
    <Suspense fallback={<div style={{ padding: 24 }}>页面加载中...</div>}>
      <Comp />
    </Suspense>
  )
}

/**
 * @param pagePath 相对 src/pages 的路径
 * 支持：
 * - 'user/list'              → ../pages/user/list.tsx 或 ../pages/user/list/index.tsx
 * - 'user/list.tsx'
 * - 'system/role/index.tsx'
 */
export function lazyLoad(pagePath: string): ReactNode {
  const normalized = pagePath.replace(/^\/+/, '').replace(/\.tsx$|\.jsx$/, '')

  const candidates = [
    `../pages/${normalized}.tsx`,
    `../pages/${normalized}/index.tsx`,
  ]

  const loader = candidates.map(key => pageModules[key]).find(Boolean)
  if (!loader) {
    console.warn(
      `[LazyLoad] 未找到页面组件: ${pagePath}\n尝试过:\n${candidates.join('\n')}`
    )
    return withSuspense(NotFoundPage)
  }
  const Comp = lazy(loader)
  return withSuspense(Comp)
}
