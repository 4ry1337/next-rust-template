import { auth } from "@/shared/lib/auth"
import { headers } from "next/headers"
import { redirect, RedirectType } from "next/navigation"

const ProtectedLayout = async ({
  children
}: Readonly<{
  children: React.ReactNode
}>) => {
  /* const session = await auth.api.getSession({
    headers: await headers()
  })
  if (!session) {
    return redirect('/signin', RedirectType.replace)
  } */
  return (
    <>
      {children}
    </>
  )
}

export default ProtectedLayout
