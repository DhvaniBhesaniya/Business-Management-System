import { AnimatePresence, motion } from 'framer-motion';
import { X } from 'lucide-react';

function cx(...parts) {
  return parts.filter(Boolean).join(' ');
}

export default function Modal({ open, onClose, title, children, className }) {
  return (
    <AnimatePresence>
      {open ? (
        <motion.div
          className="fixed inset-0 z-50"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
        >
          <div
            className="absolute inset-0 bg-black/60"
            onClick={onClose}
            role="presentation"
          />
          <motion.div
            initial={{ opacity: 0, y: 10, scale: 0.98 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: 10, scale: 0.98 }}
            transition={{ duration: 0.18 }}
            className={cx(
              'absolute left-1/2 top-1/2 w-[92vw] max-w-xl -translate-x-1/2 -translate-y-1/2 glass rounded-2xl',
              className,
            )}
            role="dialog"
            aria-modal="true"
          >
            <div className="flex items-center justify-between gap-4 px-5 pt-5 pb-3 border-b border-white/10">
              <div className="text-base font-semibold text-slate-100">{title}</div>
              <button
                type="button"
                onClick={onClose}
                className="h-9 w-9 rounded-xl hover:bg-white/10 inline-flex items-center justify-center transition"
                aria-label="Close"
              >
                <X className="h-5 w-5" />
              </button>
            </div>
            <div className="px-5 py-4">{children}</div>
          </motion.div>
        </motion.div>
      ) : null}
    </AnimatePresence>
  );
}

