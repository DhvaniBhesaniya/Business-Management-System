/**
 * @param {unknown} err
 * @returns {string}
 */
export function getErrorMessage(err) {
  const msg =
    err?.response?.data?.error ||
    err?.message ||
    (typeof err === 'string' ? err : null);
  return msg || 'Something went wrong';
}

/**
 * @param {number} n
 * @returns {string}
 */
export function formatNumber(n) {
  try {
    return new Intl.NumberFormat().format(n);
  } catch {
    return String(n);
  }
}

