'use client';

import { useState, useEffect } from 'react';
import io, { Socket } from 'socket.io-client';

interface Finding {
  id: string;
  severity: string;
  type: string;
  description: string;
  confidence: number;
}

export default function BSAPDashboard() {
  const [socket, setSocket] = useState<Socket | null>(null);
  const [scanType, setScanType] = useState('memory');
  const [target, setTarget] = useState('');
  const [findings, setFindings] = useState<Finding[]>([]);
  const [status, setStatus] = useState('');
  const [riskScore, setRiskScore] = useState(0);

  useEffect(() => {
    const newSocket = io('http://localhost:3000');
    setSocket(newSocket);

    newSocket.on('scan-progress', (data) => setStatus(data.status));
    newSocket.on('scan-complete', (data) => {
      setFindings(data.findings || []);
      setRiskScore(data.riskScore || 0);
      setStatus('completed');
    });
    newSocket.on('scan-error', (data) => setStatus(`Error: ${data.message}`));

    return () => newSocket.close();
  }, []);

  const startScan = () => {
    if (!socket || !target) return;
    const scanId = `scan-${Date.now()}`;
    socket.emit('start-scan', { scanId, scanType, target });
  };

  return (
    <div className="min-h-screen bg-zinc-950 text-white p-8">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-5xl font-bold mb-2">BSAP Dashboard</h1>
        <p className="text-zinc-400 mb-8">Backend Security Analysis Platform</p>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Scan Controls */}
          <div className="lg:col-span-1 bg-zinc-900 p-6 rounded-xl">
            <h2 className="text-xl font-semibold mb-4">Start New Scan</h2>
            
            <select 
              value={scanType} 
              onChange={(e) => setScanType(e.target.value)}
              className="w-full mb-4 bg-zinc-800 border border-zinc-700 rounded-lg p-3 text-white"
            >
              <option value="memory">Memory Corruption</option>
              <option value="malware">Malware Analysis</option>
              <option value="api">API Security</option>
              <option value="container">Container Security</option>
              <option value="runtime">Runtime Analysis</option>
              <option value="dependency">Dependency Scan</option>
            </select>

            <input
              type="text"
              value={target}
              onChange={(e) => setTarget(e.target.value)}
              placeholder="Target: /path/to/binary or URL"
              className="w-full mb-4 bg-zinc-800 border border-zinc-700 rounded-lg p-3 text-white"
            />

            <button
              onClick={startScan}
              className="w-full bg-blue-600 hover:bg-blue-700 py-3 rounded-lg font-medium transition-colors"
            >
              Start Scan
            </button>
          </div>

          {/* Results */}
          <div className="lg:col-span-2 space-y-6">
            <div className="bg-zinc-900 p-6 rounded-xl">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-2xl font-semibold">Live Results</h2>
                <div className={`px-4 py-1 rounded-full text-sm ${riskScore > 70 ? 'bg-red-500' : 'bg-emerald-500'}`}>
                  Risk: {riskScore.toFixed(0)}
                </div>
              </div>
              <p className="text-zinc-400 mb-4">Status: {status}</p>

              <div className="space-y-3">
                {findings.map((finding, i) => (
                  <div key={i} className="border border-zinc-700 p-4 rounded-lg">
                    <div className="flex justify-between">
                      <span className={`font-mono px-3 py-1 rounded text-xs ${finding.severity === 'critical' ? 'bg-red-500' : 'bg-amber-500'}`}>
                        {finding.severity.toUpperCase()}
                      </span>
                      <span className="text-emerald-400">{finding.confidence * 100}%</span>
                    </div>
                    <p className="font-semibold mt-2">{finding.type}</p>
                    <p className="text-zinc-400 text-sm mt-1">{finding.description}</p>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
