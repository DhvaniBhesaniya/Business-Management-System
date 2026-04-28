import { Loader2 } from 'lucide-react';

function cx(...parts) {
  return parts.filter(Boolean).join(' ');
}

export default function Button({
  variant = 'primary',
  size = 'md',
  loading = false,
  disabled,
  className,
  children,
  ...props
}) {
  const isDisabled = disabled || loading;

  const base =
    'inline-flex items-center justify-center gap-2 rounded-xl font-medium transition focus:outline-none focus-visible:ring-2 focus-visible:ring-violet-400/70 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-950 disabled:opacity-60 disabled:pointer-events-none';

  const variants = {
    primary:
      'bg-violet-500 text-white hover:bg-violet-400 active:bg-violet-500/90 shadow-sm',
    secondary:
      'bg-white/10 text-slate-100 hover:bg-white/15 active:bg-white/10 border border-white/10',
    ghost: 'bg-transparent text-slate-100 hover:bg-white/10 active:bg-white/10',
    danger:
      'bg-rose-500 text-white hover:bg-rose-400 active:bg-rose-500/90 shadow-sm',
  };

  const sizes = {
    sm: 'h-9 px-3 text-sm',
    md: 'h-10 px-4 text-sm',
    lg: 'h-11 px-5 text-base',
  };

  return (
    <button
      type="button"
      disabled={isDisabled}
      className={cx(base, variants[variant], sizes[size], className)}
      {...props}
    >
      {loading ? <Loader2 className="h-4 w-4 animate-spin" /> : null}
      {children}
    </button>
  );
}

