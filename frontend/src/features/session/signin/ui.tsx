'use client'

import { zodResolver } from "@hookform/resolvers/zod"
import Link from "next/link"
import { useState } from "react"
import { useForm } from "react-hook-form"
import { toast } from "sonner"
import { SignIn } from "./types"
import { SignInSchema } from "./contracts"
import { authClient } from "@/shared/lib/auth-client"
import { Form, FormField, FormItem, FormControl, FormLabel, FormMessage } from "@/shared/ui/form"
import { Input } from "@/shared/ui/input"
import { Button } from "@/shared/ui/button"
import { Separator } from "@/shared/ui/separator"

export const SignInForm = () => {
  const [loading, setLoading] = useState(false)

  const form = useForm<SignIn>({
    mode: "onTouched",
    resolver: zodResolver(SignInSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  })

  const onSubmit = async ({ email, password }: SignIn) => {
    setLoading(true)
    try {
      await authClient.signIn.email({
        email,
        password,
      })
    } catch {
      toast("Failed to sign in to the account. Please try again.")
    } finally {
      setLoading(false)
    }
  }


  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="flex flex-col gap-6">
        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Email</FormLabel>
              <FormControl>
                <Input id="email" type="email" placeholder="m@example.com" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="password"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Password</FormLabel>
              <FormControl>
                <Input id="password" type="password"  {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button disabled={loading} type="submit">
          {loading ? "Log in..." : "Log in"}
        </Button>
      </form>
    </Form >
  )
}

export default SignInForm
