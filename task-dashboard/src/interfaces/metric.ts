export interface Metric {
  memory: Memory;
  name: string;
  host: string;
  timestamp: number;
  cpu: Cpu;
}

export interface Memory {
  used: number;
  free: number;
  total: number;
}

export interface Cpu {
  usage: number;
}
