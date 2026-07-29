import { Link } from 'react-router'

export default function NotFoundPage() {
  return (
    <div style={{ padding: 24 }}>
      <h1>404</h1>
      <p>页面不存在或组件未配置</p>
      <Link to="/">返回首页</Link>
    </div>
  )
}
