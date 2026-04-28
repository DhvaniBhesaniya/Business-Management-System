export default function PageWrapper({ title, subtitle, actions, children }) {
  return (
    <div className="h-full">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between mb-5">
        <div>
          <div className="text-xl font-semibold text-slate-100">{title}</div>
          {subtitle ? (
            <div className="text-sm text-slate-300 mt-1">{subtitle}</div>
          ) : null}
        </div>
        {actions ? <div className="flex items-center gap-2">{actions}</div> : null}
      </div>
      {children}
    </div>
  );
}

