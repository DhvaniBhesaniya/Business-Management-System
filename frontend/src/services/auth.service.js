import { api } from './api';

/**
 * @typedef {{ email: string, password: string }} LoginInput
 * @typedef {{ email: string, password: string, name: string }} RegisterInput
 * @typedef {{ token: string, user: any }} LoginResponse
 */

/**
 * Authentication service.
 */
export const authService = {
  /**
   * @param {LoginInput} input
   * @returns {Promise<LoginResponse>}
   */
  async login(input) {
    const res = await api.post('/api/auth/login', input);
    return res.data;
  },

  /**
   * @param {RegisterInput} input
   * @returns {Promise<LoginResponse>}
   */
  async register(input) {
    const res = await api.post('/api/auth/register', input);
    return res.data;
  },

  /**
   * @returns {Promise<any>}
   */
  async me() {
    const res = await api.get('/api/auth/me');
    return res.data;
  },
};

