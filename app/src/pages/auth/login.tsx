import { GalleryVerticalEnd } from 'lucide-react'
import LoginForm from '@/components/auth/Login'

import LoginBg from '@/assets/images/login-bg.png'

const LoginPage = () => {
  return (
    <div className="relative grid min-h-svh w-full overflow-hidden lg:grid-cols-2">
      <div className="flex flex-col gap-4 bg-[rgba(224,241,238,0.72)] p-6 md:p-10">
        <div className="flex justify-center gap-2 md:justify-start">
          <a href="#" className="flex items-center gap-2 font-medium">
            <div className="bg-primary text-primary-foreground flex size-6 items-center justify-center rounded-md">
              <GalleryVerticalEnd className="size-4" />
            </div>
            Axum React Admin.
          </a>
        </div>
        <div className="flex flex-1 items-center justify-center">
          <div className="w-full max-w-xs">
            <LoginForm />
          </div>
        </div>
      </div>
      <div className="relative hidden bg-transparent lg:block">
        <img
          src={LoginBg}
          alt="背景图"
          className="absolute inset-0 h-full w-full object-fill"
        />
      </div>
    </div>
  )
}

export default LoginPage
