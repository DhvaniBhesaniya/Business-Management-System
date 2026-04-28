function cx(...parts) {
  return parts.filter(Boolean).join(' ');
}

export default function Card({ className, children }) {
  return <div className={cx('glass rounded-2xl', className)}>{children}</div>;
}

export function CardHeader({ className, children }) {
  return (
    <div className={cx('px-5 pt-5 pb-3 border-b border-white/10', className)}>
      {children}
    </div>
  );
}

export function CardBody({ className, children }) {
  return <div className={cx('px-5 py-4', className)}>{children}</div>;
}

