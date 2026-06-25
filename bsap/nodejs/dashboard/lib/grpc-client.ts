import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';
import path from 'path';

const PROTO_PATH = path.resolve(process.cwd(), '../../proto/bsap.proto');

const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
});

const bsapProto = grpc.loadPackageDefinition(packageDefinition).bsap;

const GRPC_SERVER = process.env.GRPC_SERVER || 'localhost:50051';

let client: any = null;

export function getGrpcClient() {
  if (!client) {
    client = new (bsapProto.ScannerService as any)(
      GRPC_SERVER,
      grpc.credentials.createInsecure() // Production: use mTLS
    );
  }
  return client;
}

export async function performScan(scanRequest: any): Promise<any> {
  const client = getGrpcClient();
  return new Promise((resolve, reject) => {
    client.PerformScan(scanRequest, (err: any, response: any) => {
      if (err) reject(err);
      else resolve(response);
    });
  });
}

// Specific scanners
export const scannerApi = {
  scanMemory: (req: any) => performScan({ ...req, scanType: 'memory' }),
  scanMalware: (req: any) => performScan({ ...req, scanType: 'malware' }),
  scanContainer: (req: any) => performScan({ ...req, scanType: 'container' }),
  scanRuntime: (req: any) => performScan({ ...req, scanType: 'runtime' }),
  // etc.
};
