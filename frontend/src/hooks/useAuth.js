import { useMutation, useQuery } from '@tanstack/react-query';
import toast from 'react-hot-toast';

import { authService } from '../services/auth.service';
import { useAuthStore } from '../store/authStore';
import { getErrorMessage } from '../utils/helpers';

/**
 * @returns {{
 *  meQuery: import('@tanstack/react-query').UseQueryResult<any, unknown>,
 *  loginMutation: import('@tanstack/react-query').UseMutationResult<any, unknown, any, unknown>,
 *  registerMutation: import('@tanstack/react-query').UseMutationResult<any, unknown, any, unknown>,
 *  logout: () => void
 * }}
 */
export function useAuth() {
  const token = useAuthStore((s) => s.token);
  const setToken = useAuthStore((s) => s.setToken);
  const setUser = useAuthStore((s) => s.setUser);
  const logout = useAuthStore((s) => s.logout);

  const meQuery = useQuery({
    queryKey: ['auth', 'me'],
    queryFn: authService.me,
    enabled: Boolean(token),
    staleTime: 60_000,
    retry: 1,
  });

  const loginMutation = useMutation({
    mutationFn: authService.login,
    onSuccess: (data) => {
      if (data?.token) setToken(data.token);
      if (data?.user) setUser(data.user);
      toast.success('Welcome back');
    },
    onError: (err) => toast.error(getErrorMessage(err)),
  });

  const registerMutation = useMutation({
    mutationFn: authService.register,
    onSuccess: (data) => {
      if (data?.token) setToken(data.token);
      if (data?.user) setUser(data.user);
      toast.success('Account created');
    },
    onError: (err) => toast.error(getErrorMessage(err)),
  });

  return { meQuery, loginMutation, registerMutation, logout };
}

