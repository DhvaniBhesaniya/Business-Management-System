import { useEffect } from 'react';

import { useAuth } from '../../hooks/useAuth';
import { useAuthStore } from '../../store/authStore';

/**
 * Bootstraps authenticated user from `/api/auth/me` when a token exists.
 * Keeps route guards from flashing incorrectly on refresh.
 */
export default function AuthBootstrap() {
  const bootstrapped = useAuthStore((s) => s.bootstrapped);
  const markBootstrapped = useAuthStore((s) => s.markBootstrapped);
  const setUser = useAuthStore((s) => s.setUser);

  const { meQuery } = useAuth();

  useEffect(() => {
    if (bootstrapped) return;

    if (meQuery.isSuccess) {
      setUser(meQuery.data);
      markBootstrapped();
      return;
    }

    if (meQuery.isError || !meQuery.isFetching) {
      // If no token, query is disabled and isFetching is false → mark done.
      markBootstrapped();
    }
  }, [
    bootstrapped,
    markBootstrapped,
    meQuery.isError,
    meQuery.isFetching,
    meQuery.isSuccess,
    meQuery.data,
    setUser,
  ]);

  return null;
}

