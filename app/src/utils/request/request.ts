import axios, { HttpStatusCode } from 'axios'
import type {
  AxiosError,
  AxiosInstance,
  AxiosRequestConfig,
  AxiosResponse,
} from 'axios'
import {
  HttpStatusMessageMap,
  type HttpRequestConfig,
  type RequestConfig,
  type Result,
} from './types'

export class HttpRequest {
  // Axios 实例对象
  instance: AxiosInstance
  // 实例基础配置
  baseConfig: AxiosRequestConfig = {
    baseURL: import.meta.env.VITE_API_BASE_URL,
    timeout: 30000,
  }

  constructor(config: HttpRequestConfig) {
    // 初始化Axios实例对象
    this.instance = axios.create({ ...this.baseConfig, ...config })

    // 请求拦截器
    this.instance.interceptors.request.use(
      (config: RequestConfig) => {
        // 获取token,并添加到请求头
        const token = localStorage.getItem('token')
        const { needToken = true } = config
        // 处理请求头的一些内容
        if (config && config.headers) {
          config.headers['Content-Type'] = 'application/json;chartset=utf-8'
          if (needToken && token) {
            config.headers.Authorization = `${token}`
          }
        }

        return config
      },
      (err: AxiosError) => {
        console.error('error', '请求发送失败')
        return Promise.reject(err)
      }
    )

    // 响应拦截器
    this.instance.interceptors.response.use(
      (res: AxiosResponse) => {
        if (res.status === HttpStatusCode.Ok) {
          return Promise.resolve(res.data)
        }
        return Promise.reject({ ...res.data })
      },
      (err: AxiosError) => {
        // 这里用来处理http常见错误，进行全局提示

        const status = err?.response?.status
        const message =
          (status != null && HttpStatusMessageMap[status]) ||
          `服务器错误(${status})!`

        console.error('[Response] ', message)
        return Promise.reject({ ...err.response, message })
      }
    )
  }

  // 自定义请求类方法
  public request(config: HttpRequestConfig): Promise<AxiosResponse> {
    return this.instance.request(config)
  }

  // * 封装常用请求方法

  public get<R, P>(
    url: string,
    params?: P,
    config?: HttpRequestConfig
  ): Promise<Result<R>> {
    return this.instance.get(url, { ...config, params })
  }

  public post<R, P>(
    url: string,
    data?: P,
    config?: HttpRequestConfig
  ): Promise<Result<R>> {
    return this.instance.post(url, data, config)
  }

  public put<R, P>(
    url: string,
    data?: P,
    config?: HttpRequestConfig
  ): Promise<Result<R>> {
    return this.instance.put(url, data, config)
  }

  public delete<R, P>(
    url: string,
    params?: P,
    config?: HttpRequestConfig
  ): Promise<Result<R>> {
    return this.instance.delete(url, { ...config, params })
  }
}

export const http = new HttpRequest({})
export const request = http.request.bind(http)
export const get = http.get.bind(http)
export const post = http.post.bind(http)
export const put = http.put.bind(http)
export const del = http.delete.bind(http)

export default http
