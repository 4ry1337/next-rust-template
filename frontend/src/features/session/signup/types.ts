import { z } from "zod"
import { SignUpSchema } from "./contracts"

export type SignUp = z.infer<typeof SignUpSchema>
