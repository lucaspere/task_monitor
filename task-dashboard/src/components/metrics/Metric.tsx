import { Box, Card, Flex, Heading, Inset } from "@radix-ui/themes";
import { MetricChart } from "./MetricChart";
import { MetricDetails } from "./MetricDetails";
import { MetricList } from "./MetricList";

export function Metric() {
  return (
    <Box minWidth="85vh" minHeight="80dvh" className="bg-gray-900">
      <Flex>
        <MetricList />
        <Flex direction="column">
          <Heading>Metric</Heading>
          <Card size="5">
            <Inset clip="padding-box" side="top" pb="current">
              <MetricChart />
            </Inset>
          </Card>
          <Card size="5">
            <MetricDetails />
          </Card>
        </Flex>
      </Flex>
    </Box>
  );
}
