import axios from 'axios';
import toast from 'react-hot-toast';

import { getAuthToken, forceLogout } from '../store/authStore';

const baseURL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

/**
 * Shared Axios instance for API requests.
 */
export const api = axios.create({
  baseURL,
  timeout: 15_000,
  headers: {
    'Content-Type': 'application/json',
  },
});

api.interceptors.request.use((config) => {
  const token = getAuthToken();
  if (token) {
    config.headers = config.headers || {};
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

api.interceptors.response.use(
  (res) => res,
  (error) => {
    const status = error?.response?.status;

    if (status === 401) {
      forceLogout();
      return Promise.reject(error);
    }

    if (status === 429) {
      toast.error('Too many requests. Please wait a moment and try again.');
      return Promise.reject(error);
    }

    return Promise.reject(error);
  },
);

