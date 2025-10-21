import { auth } from "@/shared/lib/auth";
import { headers } from "next/headers";
import Link from "next/link";
import { redirect, RedirectType } from "next/navigation";

const AuthLayout = async ({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) => {
  const session = await auth.api.getSession({
    headers: await headers()
  })
  if (session) {
    return redirect('/', RedirectType.replace)
  }
  const company_name = "Company Name"
  const company_slogan = "Company Slogan"
  return (
    <div className="grid min-h-svh lg:grid-cols-2">
      <div className="hidden flex-col justify-between bg-muted p-10 lg:flex">
        <Link href={"/"}>
          <div className="flex items-center text-lg font-medium text-white">
            <span className="">{company_name}</span>
          </div>
        </Link>
        <blockquote className="space-y-2">
          <p className="text-lg">
            &ldquo;{company_slogan}&rdquo;
          </p>
        </blockquote>
      </div>
      <div className="py-10 lg:p-10">
        <div className="mx-auto max-w-96">
          {children}
        </div>
      </div>
    </div>
  )
}

export default AuthLayout

