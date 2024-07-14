import { CallContext, createServer } from "nice-grpc";
import { firestore } from "./database/firestore";
import {
  DeepPartial,
  SystemInfoCreateRequest,
  SystemInfoCreateResponse,
  SystemInfoGetRequest,
  SystemInfoGetResponse,
  SystemMonitorDefinition,
  SystemMonitorServiceImplementation,
} from "./generated/sysinfo";
import { SystemInfo } from "./models/SystemInfo";
const sendSystemInfo: SystemMonitorServiceImplementation = {
  getSystemInfo: function (
    request: SystemInfoGetRequest,
    context: CallContext
  ): Promise<DeepPartial<SystemInfoGetResponse>> {
    throw new Error("Function not implemented.");
  },
  sendSystemInfo: async function (
    request: AsyncIterable<SystemInfoCreateRequest>,
    context: CallContext
  ): Promise<DeepPartial<SystemInfoCreateResponse>> {
    const sysinfo = new SystemInfo(firestore);
    try {
      for await (const req of request) {
        if (req.context) {
          console.log(`Received context: ${req.context.startTime}`);
          await sysinfo.start_save(req.context);
        }
        if (req.data) {
          console.log(`Received data: ${req.data}`);
          const json = new TextDecoder().decode(req.data).toString();
          const sysinfoData = SystemInfo.fromJSON(json);
          await sysinfo.save(sysinfoData);
        }
      }

      return {};
    } catch (err) {
      console.error("Erro ao processar request gRPC:", err);
      return {};
    }
  },
};

const server = createServer();

server.add(SystemMonitorDefinition, sendSystemInfo);

server
  .listen("0.0.0.0:50051")
  .then((port) => {
    console.log(`Server listening on port ${port}`);
  })
  .catch((err) => {
    console.error(err);
  });
