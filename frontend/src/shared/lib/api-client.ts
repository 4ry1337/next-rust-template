import { decodeJwt } from "jose"
import { authClient } from "./auth-client"

const API_URL = process.env.API_URL || "http://localhost:8000"

export interface ApiResponse<T = unknown> {
  data?: T
  error?: string
  status: number
}

export interface UserProfile {
  id: string
  email: string
  name: string
}

export interface AuthVerifyResponse {
  valid: boolean
  userId?: string
  email?: string
}

class ApiClient {
  private baseUrl: string
  private cachedToken: string | null

  constructor(baseUrl: string = API_URL) {
    this.baseUrl = baseUrl
    this.cachedToken = null
  }

  private validate_token(): boolean {
    if (!this.cachedToken) {
      return false
    }

    const jwt = decodeJwt(this.cachedToken)

    if (!jwt.exp) return false

    const currentTimeInSeconds = Math.floor(Date.now() / 1000);

    return jwt.exp > currentTimeInSeconds + 10
  }

  private async get_token(): Promise<string | null> {
    if (this.validate_token()) {
      return this.cachedToken
    }

    const token = await authClient.token().then(x => x.data?.token) || null

    this.cachedToken = token

    return token
  }

  private async request<T = unknown>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<ApiResponse<T>> {
    try {
      const token = await this.get_token()

      // Send request
      const response = await fetch(`${this.baseUrl}${endpoint}`, {
        ...options,
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${token}`,
          ...options.headers,
        },
      })

      const data = await response.json()

      if (!response.ok) {
        return {
          error: data.message || `API Error: ${response.status} ${response.statusText}`,
          status: response.status,
        }
      }

      return {
        data,
        status: response.status,
      }
    } catch (error) {
      return {
        error: error instanceof Error ? error.message : "Unknown error occurred",
        status: 0,
      }
    }
  }

  async verify_auth(): Promise<ApiResponse<AuthVerifyResponse>> {
    return this.request("/api/v1/auth/verify", {
      method: "GET",
    })
  }
}

export const api_cleint = new ApiClient()
export default ApiClient
