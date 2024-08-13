import { Badge, DataList } from "@radix-ui/themes";

export function MetricDetails() {
  return (
    <DataList.Root>
      <DataList.Item align="center">
        <DataList.Label minWidth="88px">Status</DataList.Label>
        <DataList.Value>
          <Badge color="jade" variant="soft" radius="full">
            Authorized
          </Badge>
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
}
