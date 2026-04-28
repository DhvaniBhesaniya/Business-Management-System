import { Navigate, Outlet, useLocation } from 'react-router-dom';

import { useAuthStore } from '../../store/authStore';

export default function ProtectedRoute() {
  const token = useAuthStore((s) => s.token);
  const bootstrapped = useAuthStore((s) => s.bootstrapped);
  const location = useLocation();

  if (!bootstrapped) {
    return (
      <div className="min-h-screen grid place-items-center">
        <div className="glass rounded-2xl px-6 py-5 text-sm text-slate-200">
          Loading…
        </div>
      </div>
    );
  }

  if (!token) {
    return <Navigate to="/login" replace state={{ from: location.pathname }} />;
  }

  return <Outlet />;
}

