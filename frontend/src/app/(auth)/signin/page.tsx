import { SignInForm } from "@/features/session/signin"
import { Button } from "@/shared/ui/button"
import Link from "next/link"

const SignInPage = () => {
  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col items-center gap-1 text-center">
        <h1 className="text-2xl font-bold">Sign in to your account</h1>
        <p className="text-muted-foreground text-sm text-balance">
          Enter your email below to sign in to your account
        </p>
      </div>
      <SignInForm />
      <div>
        Don&apos;t have an account?
        <Button asChild variant="link">
          <Link href="/signup">
            Sign Up
          </Link>
        </Button>
      </div>
    </div>
  )
}

export default SignInPage
