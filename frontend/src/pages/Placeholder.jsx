import PageWrapper from '../components/layout/PageWrapper';
import Card, { CardBody } from '../components/ui/Card';

export default function Placeholder({ title, subtitle }) {
  return (
    <PageWrapper title={title} subtitle={subtitle}>
      <Card>
        <CardBody>
          <div className="text-sm text-slate-300">Coming soon.</div>
        </CardBody>
      </Card>
    </PageWrapper>
  );
}

