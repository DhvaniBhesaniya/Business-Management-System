function cx(...parts) {
  return parts.filter(Boolean).join(' ');
}

export default function Input({ label, error, className, ...props }) {
  return (
    <label className="block">
      {label ? (
        <div className="mb-1.5 text-sm font-medium text-slate-200">{label}</div>
      ) : null}
      <input
        className={cx(
          'w-full h-10 rounded-xl bg-white/5 border border-white/10 px-3 text-sm text-slate-100 placeholder:text-slate-400 outline-none focus:ring-2 focus:ring-violet-400/60 focus:border-violet-400/40 transition',
          error ? 'border-rose-400/50 focus:ring-rose-400/40' : '',
          className,
        )}
        {...props}
      />
      {error ? (
        <div className="mt-1.5 text-xs text-rose-300">{error}</div>
      ) : null}
    </label>
  );
}

