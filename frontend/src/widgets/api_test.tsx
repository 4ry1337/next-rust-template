"use client"

import { api_cleint } from "@/shared/lib/api-client"
import { authClient } from "@/shared/lib/auth-client"
import { Button } from "@/shared/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/shared/ui/card"
import { useState } from "react"

export function APITest() {
  const { data: session } = authClient.useSession()
  const [apiResponse, setApiResponse] = useState<string>("")
  const [apiLoading, setApiLoading] = useState(false)

  const testAPI = async () => {
    setApiLoading(true)
    setApiResponse("")

    const response = await api_cleint.verify_auth()

    if (response.error) {
      setApiResponse(`❌ API Error: ${response.error}`)
    } else {
      setApiResponse(`✅ API Success: ${JSON.stringify(response.data, null, 2)}`)
    }

    setApiLoading(false)
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>API Auth</CardTitle>
        <CardDescription className="text-zinc-400">
          Test authentication against a API server
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <Button
          onClick={testAPI}
          disabled={!session || apiLoading}
          className="w-full bg-blue-600 hover:bg-blue-700 text-white"
        >
          {apiLoading ? "Testing..." : "Test API"}
        </Button>

        {!session && (
          <p className="text-zinc-400 text-sm">
            Sign in to test the API
          </p>
        )}

        {apiResponse && (
          <div className="p-4 rounded-lg">
            <h3 className="font-semibold text-white mb-2">API Response</h3>
            <pre className="text-xs whitespace-pre-wrap">
              {apiResponse}
            </pre>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
