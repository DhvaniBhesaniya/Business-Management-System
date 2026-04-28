function cx(...parts) {
  return parts.filter(Boolean).join(' ');
}

export default function Badge({ tone = 'neutral', className, children }) {
  const tones = {
    neutral: 'bg-white/10 text-slate-200 border-white/10',
    success: 'bg-emerald-500/15 text-emerald-200 border-emerald-400/20',
    warning: 'bg-amber-500/15 text-amber-200 border-amber-400/20',
    danger: 'bg-rose-500/15 text-rose-200 border-rose-400/20',
    violet: 'bg-violet-500/15 text-violet-200 border-violet-400/20',
    indigo: 'bg-indigo-500/15 text-indigo-200 border-indigo-400/20',
  };

  return (
    <span
      className={cx(
        'inline-flex items-center rounded-full border px-2.5 py-1 text-xs font-medium',
        tones[tone],
        className,
      )}
    >
      {children}
    </span>
  );
}

