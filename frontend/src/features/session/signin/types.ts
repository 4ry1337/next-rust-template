import { z } from "zod"
import { SignInSchema } from "./contracts"

export type SignIn = z.infer<typeof SignInSchema>
