import { STORAGE_KEYS } from './constants';

/**
 * @typedef {'light'|'dark'} Theme
 */

/**
 * @param {Theme} theme
 */
export function applyTheme(theme) {
  const root = document.documentElement;
  if (theme === 'dark') root.classList.add('dark');
  else root.classList.remove('dark');
}

/**
 * @returns {Theme}
 */
export function getThemeFromStorage() {
  const stored = localStorage.getItem(STORAGE_KEYS.theme);
  if (stored === 'light' || stored === 'dark') return stored;
  return 'dark';
}

/**
 * Loads theme from localStorage and applies it.
 * @returns {Theme}
 */
export function loadThemeFromStorage() {
  const theme = getThemeFromStorage();
  applyTheme(theme);
  return theme;
}

/**
 * @param {Theme} theme
 */
export function setTheme(theme) {
  localStorage.setItem(STORAGE_KEYS.theme, theme);
  applyTheme(theme);
}

