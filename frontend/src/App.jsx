import { AnimatePresence, motion } from 'framer-motion';
import { Navigate, Route, Routes, useLocation } from 'react-router-dom';

import ProtectedRoute from './components/layout/ProtectedRoute';
import PublicOnlyRoute from './components/layout/PublicOnlyRoute';
import AppShell from './components/layout/AppShell';
import AuthBootstrap from './components/layout/AuthBootstrap';

import Login from './pages/Auth/Login.jsx';
import Register from './pages/Auth/Register.jsx';
import Dashboard from './pages/Dashboard/index.jsx';
import Placeholder from './pages/Placeholder.jsx';

function AnimatedPage({ children }) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -8 }}
      transition={{ duration: 0.2 }}
      className="h-full"
    >
      {children}
    </motion.div>
  );
}

export default function App() {
  const location = useLocation();

  return (
    <>
      <AuthBootstrap />
      <AnimatePresence mode="wait">
        <Routes location={location} key={location.pathname}>
          <Route element={<PublicOnlyRoute />}>
            <Route
              path="/login"
              element={
                <AnimatedPage>
                  <Login />
                </AnimatedPage>
              }
            />
            <Route
              path="/register"
              element={
                <AnimatedPage>
                  <Register />
                </AnimatedPage>
              }
            />
          </Route>

          <Route element={<ProtectedRoute />}>
            <Route element={<AppShell />}>
              <Route
                path="/dashboard"
                element={
                  <AnimatedPage>
                    <Dashboard />
                  </AnimatedPage>
                }
              />
              <Route
                path="/users"
                element={
                  <AnimatedPage>
                    <Placeholder title="Users" subtitle="User management will be implemented next." />
                  </AnimatedPage>
                }
              />
              <Route
                path="/products"
                element={
                  <AnimatedPage>
                    <Placeholder
                      title="Products"
                      subtitle="Product management will be implemented next."
                    />
                  </AnimatedPage>
                }
              />
              <Route
                path="/settings"
                element={
                  <AnimatedPage>
                    <Placeholder
                      title="Settings"
                      subtitle="Profile + password + preferences will be implemented next."
                    />
                  </AnimatedPage>
                }
              />
              <Route path="/" element={<Navigate to="/dashboard" replace />} />
            </Route>
          </Route>

          <Route path="*" element={<Navigate to="/dashboard" replace />} />
        </Routes>
      </AnimatePresence>
    </>
  );
}
