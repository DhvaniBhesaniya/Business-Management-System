import { useMemo, useState } from 'react';
import { Outlet, useLocation } from 'react-router-dom';
import { AnimatePresence, motion } from 'framer-motion';
import { LogOut } from 'lucide-react';

import Sidebar from './Sidebar';
import Topbar from './Topbar';
import Button from '../ui/Button';
import { useAuthStore } from '../../store/authStore';

export default function AppShell() {
  const [collapsed, setCollapsed] = useState(false);
  const [mobileOpen, setMobileOpen] = useState(false);
  const user = useAuthStore((s) => s.user);
  const logout = useAuthStore((s) => s.logout);
  const location = useLocation();

  const headerRight = useMemo(() => {
    return (
      <div className="flex items-center gap-2">
        <div className="hidden sm:block text-right">
          <div className="text-sm font-medium text-slate-100 leading-5">
            {user?.name || '—'}
          </div>
          <div className="text-xs text-slate-300">{user?.role || ''}</div>
        </div>
        <Button
          variant="secondary"
          className="h-10"
          onClick={() => {
            logout();
            window.location.assign('/login');
          }}
        >
          <LogOut className="h-4 w-4" />
          <span className="hidden sm:inline">Logout</span>
        </Button>
      </div>
    );
  }, [logout, user?.name, user?.role]);

  return (
    <div className="min-h-screen">
      <div className="grid grid-cols-1 sm:grid-cols-[auto_1fr]">
        <div className="hidden sm:block h-screen sticky top-0">
          <div
            className={[
              'h-full',
              collapsed ? 'w-[92px]' : 'w-[280px]',
              'transition-[width] duration-200',
            ].join(' ')}
          >
            <div className="h-full">
              <div className="px-4 pt-4">
                <div className="glass rounded-2xl overflow-hidden">
                  <div className="flex items-center justify-between px-4 py-3 border-b border-white/10">
                    <div className="text-sm font-semibold text-slate-100">
                      {!collapsed ? 'Navigation' : 'Nav'}
                    </div>
                    <Button
                      variant="ghost"
                      className="h-9 w-9 px-0"
                      onClick={() => setCollapsed((v) => !v)}
                      aria-label="Toggle sidebar"
                      title="Toggle sidebar"
                    >
                      <div className="h-2 w-2 rounded-full bg-white/60" />
                    </Button>
                  </div>
                  <div className="h-[calc(100vh-90px)]">
                    <Sidebar collapsed={collapsed} />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="min-h-screen">
          <Topbar onOpenMobile={() => setMobileOpen(true)} right={headerRight} />

          <main className="px-4 sm:px-6 pb-8 pt-6">
            <div className="max-w-6xl mx-auto">
              <AnimatePresence mode="wait">
                <motion.div
                  key={location.pathname}
                  initial={{ opacity: 0, y: 8 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -8 }}
                  transition={{ duration: 0.2 }}
                >
                  <Outlet />
                </motion.div>
              </AnimatePresence>
            </div>
          </main>
        </div>
      </div>

      <AnimatePresence>
        {mobileOpen ? (
          <motion.div
            className="fixed inset-0 z-40 sm:hidden"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
          >
            <div
              className="absolute inset-0 bg-black/60"
              onClick={() => setMobileOpen(false)}
              role="presentation"
            />
            <motion.div
              className="absolute left-0 top-0 bottom-0 w-[84vw] max-w-[320px] p-4"
              initial={{ x: -30, opacity: 0 }}
              animate={{ x: 0, opacity: 1 }}
              exit={{ x: -30, opacity: 0 }}
              transition={{ duration: 0.18 }}
            >
              <div className="glass rounded-2xl h-full overflow-hidden">
                <div className="px-4 py-3 border-b border-white/10">
                  <div className="text-sm font-semibold text-slate-100">Menu</div>
                </div>
                <div className="h-[calc(100%-52px)]">
                  <Sidebar
                    collapsed={false}
                    onNavigate={() => setMobileOpen(false)}
                  />
                </div>
              </div>
            </motion.div>
          </motion.div>
        ) : null}
      </AnimatePresence>
    </div>
  );
}

