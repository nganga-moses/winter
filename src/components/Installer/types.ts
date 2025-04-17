export type Mode = "local" | "cloud" | null;

export interface ModelOption {
    name: string;
    quant: string;
    sizeGb: number;
    url: string;
    checksum: string;
}