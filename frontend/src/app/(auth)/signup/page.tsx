import { SignUpForm } from "@/features/session/signup"
import { Button } from "@/shared/ui/button"
import Link from "next/link"

const SignUpPage = () => {
  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col items-center gap-1 text-center">
        <h1 className="text-2xl font-bold">Sign up to your account</h1>
        <p className="text-muted-foreground text-sm text-balance">
          Enter your email below to sign up to your account
        </p>
      </div>
      <SignUpForm />
      <div>
        Already have an account?
        <Button asChild variant="link">
          <Link href="/signin">
            Sign In
          </Link>
        </Button>
      </div>
    </div>
  )
}

export default SignUpPage
