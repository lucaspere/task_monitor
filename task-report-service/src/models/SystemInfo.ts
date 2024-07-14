import {
  CollectionReference,
  DocumentReference,
  Firestore,
} from "@google-cloud/firestore";
import { SystemInfoContext } from "../generated/sysinfo";
export class CpuMetrics {
  /**
   * The CPU usage in percentage.
   */
  public usage: number;

  constructor(usage: number) {
    this.usage = usage;
  }
}

export class MemoryMetrics {
  /**
   * The total memory in bytes.
   */
  public total: number;
  /**
   * The memory used in bytes.
   */
  public used: number;
  /**
   * The memory free in bytes.
   */
  public free: number;

  constructor(total: number, used: number, free: number) {
    this.total = total;
    this.used = used;
    this.free = free;
  }
}

export class SystemMetrics {
  public cpu: CpuMetrics;
  public memory: MemoryMetrics;

  constructor(cpu: CpuMetrics, memory: MemoryMetrics) {
    this.cpu = cpu;
    this.memory = memory;
  }
}

export class SystemInfo {
  static CollectionName: string = "system_info";
  private collection: CollectionReference = {} as CollectionReference;
  private doc: DocumentReference = {} as DocumentReference;
  /**
   * The name of the system.
   */
  public name: string = "";
  /**
   * The host name of the system.
   */
  public host: string = "";
  /**
   * The timestamp of the metrics.
   */
  public timestamp: number = 0;
  /**
   * The CPU metrics.
   */
  public cpu: CpuMetrics = new CpuMetrics(0);
  /**
   * The memory metrics.
   */
  public memory: MemoryMetrics = new MemoryMetrics(0, 0, 0);

  constructor(firestore: Firestore) {
    this.collection = firestore.collection(SystemInfo.CollectionName);
  }
  async start_save(ctx: SystemInfoContext) {
    this.doc = this.collection.doc(ctx.startTime);

    await this.doc.set(ctx);
  }

  async save(sysinfo: SystemInfo): Promise<void> {
    await this.doc
      .collection("metrics")
      .doc(sysinfo.timestamp.toString())
      .set(sysinfo);
  }

  static fromJSON(json: string) {
    let obj = JSON.parse(json);
    const sysinfo = {
      name: obj.name,
      host: obj.host,
      timestamp: obj.timestamp,
      cpu: obj.cpu,
      memory: obj.memory,
    } as unknown as SystemInfo;

    return sysinfo;
  }
}
