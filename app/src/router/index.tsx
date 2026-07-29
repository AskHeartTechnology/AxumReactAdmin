import { createBrowserRouter, Navigate } from 'react-router'
import { GuestOnly, RequireAuth } from './guards'
import { createAppRoutes, createLoginRoute } from './routes'

import AppLayout from '@/components/layout'
import NotFoundPage from '@/pages/auth/404'

export const router = createBrowserRouter([
  {
    element: <GuestOnly />,
    children: [createLoginRoute()],
  },
  {
    element: <RequireAuth />,
    children: [
      {
        path: '/',
        element: <AppLayout />,
        children: [
          ...createAppRoutes(),
          { path: '*', element: <NotFoundPage /> },
        ],
      },
    ],
  },
  { path: '*', element: <Navigate to="/" replace /> },
])
