import { Navigate, Outlet } from 'react-router-dom';

import { useAuthStore } from '../../store/authStore';

export default function PublicOnlyRoute() {
  const token = useAuthStore((s) => s.token);
  const bootstrapped = useAuthStore((s) => s.bootstrapped);

  if (!bootstrapped) return <Outlet />;

  if (token) return <Navigate to="/dashboard" replace />;

  return <Outlet />;
}

