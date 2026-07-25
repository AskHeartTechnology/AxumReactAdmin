import type { InternalAxiosRequestConfig } from 'axios'

// 自定义code码
export const ResponseCode = {
  SUCCESS: 2000, // 成功
  BAD_REQUEST: 4000, // 请求错误
  UNAUTHORIZED: 4001, // 未认证
  FORBIDDEN: 4003, // 请求被拒绝
  NOT_FOUND: 4004, // 请求地址不存在
  ERROR: 5000, // 服务器异常
} as const

export const HttpStatusMessageMap: Record<number, string> = {
  400: '请求错误(400)',
  401: '未授权，请重新登录(401)',
  403: '拒绝访问(403)',
  404: '请求地址不存在(404)',
  408: '请求超时(408)',
  500: '服务器错误(500)',
  501: '服务未实现(501)',
  502: '网络错误(502)',
  503: '服务不可用(503)',
  504: '网络超时(504)',
  505: 'HTTP版本不受支持(505)',
}

export type HttpResponseCode = (typeof ResponseCode)[keyof typeof ResponseCode]

export interface RequestConfig extends InternalAxiosRequestConfig {
  loading?: boolean
  needToken?: boolean
}

export type HttpRequestConfig = Omit<RequestConfig, 'headers'>

export type Result<T> = {
  code: HttpResponseCode
  message: string
  result: T
}
