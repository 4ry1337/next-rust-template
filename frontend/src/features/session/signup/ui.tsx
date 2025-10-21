'use client'

import { useState } from "react"
import Link from "next/link"
import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { toast } from "sonner"
import { Form, FormField, FormItem, FormControl, FormLabel, FormMessage } from "@/shared/ui/form"
import { Input } from "@/shared/ui/input"
import { Button } from "@/shared/ui/button"
import { Separator } from "@/shared/ui/separator"
import { SignUp } from "./types"
import { SignUpSchema } from "./contracts"
import { authClient } from "@/shared/lib/auth-client"

export const SignUpForm = () => {
  const [loading, setLoading] = useState(false)

  const form = useForm<SignUp>({
    resolver: zodResolver(SignUpSchema),
    defaultValues: {
      name: "",
      email: "",
      password: "",
    },
  })

  const onSubmit = async ({ email, name, password }: SignUp) => {
    setLoading(true)
    try {
      await authClient.signUp.email({
        email,
        name,
        password,
      })
    } catch {
      toast("Failed to create account. Please try again.")
    } finally {
      setLoading(false)
    }
  }


  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="flex flex-col gap-6">
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Full Name</FormLabel>
              <FormControl>
                <Input id="name" type="text" placeholder="Rakhat Yskak" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
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
          {loading ? "Creating Account..." : "Create Account"}
        </Button>
      </form>
    </Form >
  )
}

export default SignUpForm
