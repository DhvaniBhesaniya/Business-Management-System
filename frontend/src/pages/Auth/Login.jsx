import { useMemo } from 'react';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';

import Card, { CardBody, CardHeader } from '../../components/ui/Card';
import Input from '../../components/ui/Input';
import Button from '../../components/ui/Button';
import { useAuth } from '../../hooks/useAuth';

const schema = z.object({
  email: z.string().email('Enter a valid email'),
  password: z.string().min(1, 'Password is required'),
});

export default function Login() {
  const navigate = useNavigate();
  const location = useLocation();
  const from = useMemo(() => location.state?.from || '/dashboard', [location.state]);

  const { loginMutation } = useAuth();

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(schema),
    defaultValues: { email: '', password: '' },
  });

  const onSubmit = async (values) => {
    await loginMutation.mutateAsync(values);
    navigate(from, { replace: true });
  };

  return (
    <div className="min-h-screen grid place-items-center px-4">
      <div className="w-full max-w-md">
        <Card>
          <CardHeader>
            <div className="text-lg font-semibold text-slate-100">Sign in</div>
            <div className="text-sm text-slate-300 mt-1">
              Access your business dashboard
            </div>
          </CardHeader>
          <CardBody>
            <form className="space-y-4" onSubmit={handleSubmit(onSubmit)}>
              <Input
                label="Email"
                type="email"
                placeholder="you@company.com"
                error={errors.email?.message}
                {...register('email')}
              />
              <Input
                label="Password"
                type="password"
                placeholder="••••••••"
                error={errors.password?.message}
                {...register('password')}
              />

              <Button
                type="submit"
                className="w-full"
                loading={loginMutation.isPending}
              >
                Sign in
              </Button>
            </form>

            <div className="mt-5 text-sm text-slate-300">
              New here?{' '}
              <Link
                to="/register"
                className="text-violet-200 hover:text-violet-100 underline underline-offset-4"
              >
                Create an account
              </Link>
            </div>
          </CardBody>
        </Card>
      </div>
    </div>
  );
}

