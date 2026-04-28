import { useQuery } from '@tanstack/react-query';

import Card, { CardBody, CardHeader } from '../../components/ui/Card';
import Skeleton from '../../components/ui/Skeleton';
import PageWrapper from '../../components/layout/PageWrapper';
import { api } from '../../services/api';
import { formatNumber } from '../../utils/helpers';

async function fetchUsersCount() {
  const res = await api.get('/api/users', { params: { page: 1, limit: 1 } });
  return res.data?.total ?? 0;
}

async function fetchProductsCount() {
  const res = await api.get('/api/products', { params: { page: 1, limit: 1 } });
  return res.data?.total ?? 0;
}

function StatCard({ title, value, loading }) {
  return (
    <Card className="overflow-hidden">
      <CardHeader>
        <div className="text-sm text-slate-300">{title}</div>
      </CardHeader>
      <CardBody>
        {loading ? (
          <Skeleton className="h-8 w-28" />
        ) : (
          <div className="text-3xl font-semibold text-slate-100">{value}</div>
        )}
      </CardBody>
    </Card>
  );
}

export default function Dashboard() {
  const usersCount = useQuery({
    queryKey: ['dashboard', 'usersCount'],
    queryFn: fetchUsersCount,
    staleTime: 30_000,
  });

  const productsCount = useQuery({
    queryKey: ['dashboard', 'productsCount'],
    queryFn: fetchProductsCount,
    staleTime: 30_000,
  });

  return (
    <PageWrapper
      title="Dashboard"
      subtitle="Overview of users and inventory (computed from existing APIs)."
    >
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        <StatCard
          title="Total Users"
          value={formatNumber(usersCount.data || 0)}
          loading={usersCount.isLoading}
        />
        <StatCard
          title="Total Products"
          value={formatNumber(productsCount.data || 0)}
          loading={productsCount.isLoading}
        />
        <StatCard title="Recent Activity" value="Coming soon" loading={false} />
      </div>

      <div className="mt-6 grid grid-cols-1 gap-4">
        <Card>
          <CardHeader>
            <div className="text-sm font-semibold text-slate-100">Analytics</div>
            <div className="text-sm text-slate-300 mt-1">
              Charts + activity feed will be wired in next modules once endpoints exist.
            </div>
          </CardHeader>
          <CardBody>
            <div className="text-sm text-slate-300">
              Placeholder panel (Module 1). We’ll add Recharts growth charts and a
              derived “recent activity” table next.
            </div>
          </CardBody>
        </Card>
      </div>
    </PageWrapper>
  );
}

