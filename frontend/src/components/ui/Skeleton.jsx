function cx(...parts) {
  return parts.filter(Boolean).join(' ');
}

export default function Skeleton({ className }) {
  return (
    <div
      className={cx(
        'animate-pulse rounded-xl bg-white/10 border border-white/10',
        className,
      )}
    />
  );
}

