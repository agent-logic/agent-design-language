"""Linux-only kernel peer-credential proof with synthetic service replies."""
import json
import os
from pathlib import Path
import socket
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch
from idle_stop import Transport, Unsafe

@unittest.skipUnless(sys.platform.startswith('linux'), 'Linux SO_PEERCRED is required')
class LinuxTransport(unittest.TestCase):
    def exchange(self, supplied_pid):
        with tempfile.TemporaryDirectory() as directory:
            path=str(Path(directory)/'control.sock')
            listener=socket.socket(socket.AF_UNIX,socket.SOCK_STREAM)
            listener.bind(path); listener.listen(1)
            received=[]
            def serve():
                with listener.accept()[0] as client:
                    value=client.recv(4096)
                    received.append(value)
                    if value: client.sendall(b'{"schema":"codefriend.host_control.v1","ok":true}\n')
            thread=threading.Thread(target=serve); thread.start()
            real_socket=socket.socket
            class Redirect:
                def __init__(self,*args,**kwargs): self.inner=real_socket(*args,**kwargs)
                def __enter__(self): return self
                def __exit__(self,*args): self.inner.close()
                def connect(self,_): self.inner.connect(path)
                def __getattr__(self,key): return getattr(self.inner,key)
            try:
                with patch('idle_stop.socket.socket',Redirect):
                    result=Transport().request('website',supplied_pid,{'schema':'codefriend.host_control.v1','action':'status'})
                return result,received
            finally:
                thread.join(3); listener.close()
                self.assertFalse(thread.is_alive())
    def test_real_kernel_peer_pid_matches(self):
        result,received=self.exchange(os.getpid())
        self.assertTrue(result['ok']); self.assertEqual(json.loads(received[0])['action'],'status')
    def test_real_kernel_peer_pid_mismatch_denies(self):
        with self.assertRaises(Unsafe): self.exchange(os.getpid()+1000000)

if __name__=='__main__': unittest.main()
