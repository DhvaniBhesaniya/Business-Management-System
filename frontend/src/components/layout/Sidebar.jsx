import { NavLink } from 'react-router-dom';
import { Box, LayoutDashboard, Settings, Users } from 'lucide-react';

function cx(...parts) {
  return parts.filter(Boolean).join(' ');
}

const nav = [
  { to: '/dashboard', label: 'Dashboard', icon: LayoutDashboard },
  { to: '/users', label: 'Users', icon: Users },
  { to: '/products', label: 'Products', icon: Box },
  { to: '/settings', label: 'Settings', icon: Settings },
];

export default function Sidebar({ collapsed, onNavigate }) {
  return (
    <div className="h-full flex flex-col">
      <div className="px-4 py-4">
        <div className="glass rounded-2xl px-4 py-3">
          <div className="flex items-center gap-3">
            <div className="h-9 w-9 rounded-xl bg-violet-500/20 border border-violet-400/20 grid place-items-center">
              <div className="h-2.5 w-2.5 rounded-full bg-violet-300" />
            </div>
            {!collapsed ? (
              <div>
                <div className="text-sm font-semibold text-slate-100 leading-5">
                  Business System
                </div>
                <div className="text-xs text-slate-300">Admin Console</div>
              </div>
            ) : null}
          </div>
        </div>
      </div>

      <div className="px-3 pb-3 flex-1 overflow-auto">
        <div className="flex flex-col gap-1">
          {nav.map((item) => {
            const Icon = item.icon;
            return (
              <NavLink
                key={item.to}
                to={item.to}
                onClick={onNavigate}
                className={({ isActive }) =>
                  cx(
                    'group flex items-center gap-3 rounded-2xl px-3 py-2.5 text-sm transition border',
                    isActive
                      ? 'bg-white/10 border-white/15 text-slate-100'
                      : 'bg-transparent border-transparent text-slate-300 hover:bg-white/5 hover:border-white/10 hover:text-slate-100',
                  )
                }
              >
                <Icon className="h-5 w-5 opacity-90" />
                {!collapsed ? <span className="font-medium">{item.label}</span> : null}
              </NavLink>
            );
          })}
        </div>
      </div>

      <div className="px-4 pb-4">
        <div className="glass rounded-2xl px-4 py-3">
          <div className={cx('text-xs text-slate-300', collapsed ? 'text-center' : '')}>
            v1.0
          </div>
        </div>
      </div>
    </div>
  );
}

