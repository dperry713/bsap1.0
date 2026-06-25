import { Server as SocketIOServer } from 'socket.io';
import { createServer } from 'http';
import { performScan } from './grpc-client';

let io: SocketIOServer;

export function initSocketServer(server: any) {
  io = new SocketIOServer(server, {
    cors: {
      origin: "*",
      methods: ["GET", "POST"]
    }
  });

  io.on('connection', (socket) => {
    console.log('Client connected:', socket.id);

    socket.on('start-scan', async (data) => {
      try {
        socket.emit('scan-progress', { status: 'started', scanId: data.scanId });

        const result = await performScan({
          scanId: data.scanId || Date.now().toString(),
          scanType: data.scanType,
          target: data.target,
          tenantId: data.tenantId || 'default'
        });

        socket.emit('scan-complete', {
          scanId: result.scanId,
          findings: result.findings,
          riskScore: result.riskScore,
          status: result.status
        });
      } catch (error) {
        socket.emit('scan-error', { message: (error as Error).message });
      }
    });

    socket.on('disconnect', () => {
      console.log('Client disconnected');
    });
  });

  return io;
}
