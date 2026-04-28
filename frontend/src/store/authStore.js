import { create } from 'zustand';
import { STORAGE_KEYS } from '../utils/constants';

/**
 * @typedef {{ id: string, email: string, name: string, role: 'admin'|'manager'|'cashier', is_active: boolean, created_at?: string }} User
 */

/**
 * @typedef {Object} AuthState
 * @property {string|null} token
 * @property {User|null} user
 * @property {boolean} bootstrapped
 * @property {(token: string) => void} setToken
 * @property {(user: User|null) => void} setUser
 * @property {() => void} markBootstrapped
 * @property {() => void} logout
 */

/**
 * Zustand auth store.
 */
export const useAuthStore = create((set) => ({
  token: localStorage.getItem(STORAGE_KEYS.token),
  user: null,
  bootstrapped: false,

  setToken: (token) => {
    localStorage.setItem(STORAGE_KEYS.token, token);
    set({ token });
  },

  setUser: (user) => set({ user }),

  markBootstrapped: () => set({ bootstrapped: true }),

  logout: () => {
    localStorage.removeItem(STORAGE_KEYS.token);
    set({ token: null, user: null, bootstrapped: true });
  },
}));

/**
 * @returns {string|null}
 */
export function getAuthToken() {
  return useAuthStore.getState().token;
}

/**
 * Logs out and redirects to /login.
 * Safe to call from non-React modules (e.g., Axios interceptors).
 */
export function forceLogout() {
  const { logout } = useAuthStore.getState();
  logout();

  if (window.location.pathname !== '/login') {
    window.location.assign('/login');
  }
}

