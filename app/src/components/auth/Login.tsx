import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Field, FieldGroup, FieldLabel } from '@/components/ui/field'
import { Input } from '@/components/ui/input'

const LoginForm = ({ className, ...props }: React.ComponentProps<'form'>) => {
  return (
    <form className={cn('flex flex-col gap-6', className)} {...props}>
      <FieldGroup>
        <div className="flex flex-col items-center gap-1 text-center">
          <h1 className="text-2xl font-bold">使用系统账户登录</h1>
          <p className="text-muted-foreground text-sm text-balance">
            在下方输入你的电子邮件地址以登录系统
          </p>
        </div>
        <Field>
          <FieldLabel htmlFor="email">邮 箱</FieldLabel>
          <Input id="email" type="email" placeholder="m@example.com" required />
        </Field>
        <Field>
          <div className="flex items-center">
            <FieldLabel htmlFor="password">密 码</FieldLabel>
            <a
              href="#"
              className="ml-auto text-sm underline-offset-4 hover:underline"
            >
              忘记密码?
            </a>
          </div>
          <Input id="password" type="password" required />
        </Field>
        <Field>
          <Button type="submit">登 录</Button>
        </Field>
      </FieldGroup>
    </form>
  )
}

export default LoginForm
