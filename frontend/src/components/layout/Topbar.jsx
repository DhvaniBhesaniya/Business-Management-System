import { Menu, Moon, Sun } from 'lucide-react';

import Button from '../ui/Button';
import { getThemeFromStorage, setTheme } from '../../utils/theme';

export default function Topbar({ onOpenMobile, right }) {
  const theme = getThemeFromStorage();
  const isDark = theme === 'dark';

  return (
    <div className="sticky top-0 z-20">
      <div className="px-4 sm:px-6 pt-4">
        <div className="glass rounded-2xl px-3 py-2 flex items-center justify-between gap-3">
          <div className="flex items-center gap-2">
            <Button
              variant="ghost"
              className="sm:hidden h-10 w-10 px-0"
              onClick={onOpenMobile}
              aria-label="Open menu"
            >
              <Menu className="h-5 w-5" />
            </Button>
            <div className="hidden sm:block text-sm text-slate-300">
              Deep Indigo • Violet Accent
            </div>
          </div>

          <div className="flex items-center gap-2">
            <Button
              variant="secondary"
              className="h-10 w-10 px-0"
              onClick={() => setTheme(isDark ? 'light' : 'dark')}
              aria-label="Toggle theme"
              title="Toggle theme"
            >
              {isDark ? <Sun className="h-5 w-5" /> : <Moon className="h-5 w-5" />}
            </Button>
            {right}
          </div>
        </div>
      </div>
    </div>
  );
}

