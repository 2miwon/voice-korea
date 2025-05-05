import axios from 'axios'
import { CONFIGS } from '../configs/index.js'

const api = axios.create({
    baseURL: CONFIGS.BASE_API_URL,
    timeout: 10000,
  })

  export const makeApiCall = async (
    url: string,
    options: {
      method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
      params?: Record<string, any>
      data?: any
      headers?: Record<string, string>
    } = {}
  ) => {
    try {
      const res = await api({
        url,
        method: options.method || 'GET',
        params: options.params,
        data: options.data,
        headers: options.headers,
      })

      return res.data
    } catch (error: any) {
      // console.error('API Error:', error)
      throw error?.message
    }
  }