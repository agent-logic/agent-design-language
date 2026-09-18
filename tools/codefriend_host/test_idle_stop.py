import copy
import unittest
from unittest.mock import patch
import struct
from idle_stop import Transport as SocketTransport
from idle_stop import Controller, SCHEMA, Unsafe

class Supervisor:
    def __init__(self):
        self.states = {s: (i+10, 'active', 'running', s+'-boot') for i,s in enumerate(('website','gateway'))}
        self.events = []
        self.fail_stop = False
    def state(self,s): return self.states[s]
    def stop(self,s):
        self.events.append(('stop',s))
        if not self.fail_stop: self.states[s]=(0,'inactive','dead',s+'-boot')
    def poweroff(self): self.events.append(('poweroff',))

class Transport:
    def __init__(self):
        self.events=[]
        self.replies={s:dict(schema=SCHEMA,ok=True,service=s,pid=i+10,candidate_revision='a'*40,
                            instance=str(i+1)*64,attempt=None,draining=False,drained_without_payloads=True,quiescent_without_payloads=True,
                            idle_seconds=1800) for i,s in enumerate(('website','gateway'))}
        self.hook=lambda s,a,r: None
    def request(self,s,pid,request):
        action=request['action']; self.events.append((s,action))
        reply=self.replies[s]
        if action!='status':
            if action=='drain': reply.update(draining=True,attempt=request['attempt'])
            elif reply['attempt']==request['attempt']: reply.update(draining=False,attempt=None)
        self.hook(s,action,reply)
        return copy.deepcopy(reply)

class Cases(unittest.TestCase):
    def setUp(self):
        self.s=Supervisor(); self.t=Transport(); self.c=Controller(self.s,self.t,dict(website='a'*40,gateway='a'*40))
    def denied(self):
        with self.assertRaises(Exception): self.c.run(True)
        self.assertNotIn(('poweroff',),self.s.events)
    def test_safe_stop_order(self):
        self.assertEqual(self.c.run(True),'poweroff_requested')
        self.assertEqual(self.s.events,[('stop','website'),('stop','gateway'),('poweroff',)])
    def test_observation_never_mutates(self):
        self.c.run(); self.assertEqual(self.t.events,[('website','status'),('gateway','status')]); self.assertEqual(self.s.events,[])
    def test_idle_boundary(self):
        for value in [1799,True,float('nan'),float('inf'),None,-1]:
            with self.subTest(value=value):
                self.setUp(); self.t.replies['website']['idle_seconds']=value; self.denied()
    def test_recent_activity_after_drain(self):
        self.t.hook=lambda s,a,r:r.update(idle_seconds=0) if s=='website' and a=='drain' else None
        self.denied(); self.assertFalse(self.t.replies['website']['draining'])
    def test_payload_or_uncertainty_defers(self):
        for service in ('website','gateway'):
            with self.subTest(service=service):
                self.setUp(); self.t.replies[service]['drained_without_payloads']=False; self.denied()
                self.assertEqual(self.s.events,[])
                self.assertFalse(self.t.replies['website']['draining'])
    def test_retention_deferrals_do_not_exhaust_attempt_capacity(self):
        for service in ('website','gateway'):
            with self.subTest(service=service):
                self.setUp()
                self.t.replies[service]['quiescent_without_payloads']=False
                for _ in range(1100): self.denied()
                self.assertTrue(all(action=='status' for _,action in self.t.events))
                self.assertEqual(self.s.events,[])
                self.t.replies[service]['quiescent_without_payloads']=True
                self.assertEqual(self.c.run(True),'poweroff_requested')
    def test_missing_quiescent_signal_denies(self):
        del self.t.replies['website']['quiescent_without_payloads']; self.denied()
    def test_work_arriving_between_preflight_and_drain_denies(self):
        def hook(s,a,r):
            if s=='gateway' and a=='drain': r['drained_without_payloads']=False
        self.t.hook=hook; self.denied(); self.assertEqual(self.s.events,[])
    def test_wrong_candidate(self):
        self.t.replies['gateway']['candidate_revision']='b'*40; self.denied()
    def test_wrong_pid(self):
        self.t.replies['website']['pid']=999; self.denied()
    def test_other_attempt_never_resumed(self):
        self.t.replies['website'].update(draining=True,attempt='b'*32); self.denied()
        self.assertEqual(self.t.events,[('website','status')])
    def test_timeout_after_effect_resumes_exact_attempt(self):
        def hook(s,a,r):
            if s=='gateway' and a=='drain': raise TimeoutError()
        self.t.hook=hook; self.denied()
        self.assertFalse(self.t.replies['gateway']['draining']); self.assertFalse(self.t.replies['website']['draining'])
    def test_instance_restart_denies_and_never_resumes_new_instance(self):
        def hook(s,a,r):
            if s=='gateway' and a=='drain':
                r['instance']='f'*64
                self.s.states[s]=(42,'active','running','new-boot')
        self.t.hook=hook; self.denied()
        self.assertNotIn(('gateway','resume'),self.t.events)
    def test_malformed_response_denies(self):
        self.t.replies['website']['ok']=False; self.denied()
    def test_stop_failure_denies(self):
        self.s.fail_stop=True; self.denied()
    def test_restart_after_first_stop_denies(self):
        original=self.s.stop
        def stop(s):
            original(s)
            if s=='gateway': self.s.states['website']=(30,'active','running','restarted')
        self.s.stop=stop; self.denied()
    def test_idle_poll_does_not_reset_clock(self):
        for _ in range(4): self.c.run(False)
        self.c.run(True); self.assertIn(('poweroff',),self.s.events)

class SocketCases(unittest.TestCase):
    class Socket:
        def __init__(self, chunks, pid=10): self.chunks=iter(chunks); self.pid=pid; self.sent=[]
        def __enter__(self): return self
        def __exit__(self,*args): pass
        def settimeout(self,value): pass
        def connect(self,path): pass
        def getsockopt(self,*args): return struct.pack('3i',self.pid,1000,1000)
        def sendall(self,value): self.sent.append(value)
        def recv(self,size): return next(self.chunks,b'')
    def call(self, fake, clock=None):
        with patch('idle_stop.socket.socket',return_value=fake), patch('idle_stop.socket.SO_PEERCRED',17,create=True):
            if clock:
                with patch('idle_stop.time.monotonic',side_effect=clock):
                    return SocketTransport().request('website',10,{'schema':SCHEMA,'action':'status'})
            return SocketTransport().request('website',10,{'schema':SCHEMA,'action':'status'})
    def test_valid_frame(self):
        self.assertEqual(self.call(self.Socket([b'{"ok":true}\n'])),{'ok':True})
    def test_wrong_peer_never_receives_command(self):
        fake=self.Socket([],pid=11)
        with self.assertRaises(Unsafe): self.call(fake)
        self.assertEqual(fake.sent,[])
    def test_oversized_frame(self):
        with self.assertRaises(Unsafe): self.call(self.Socket([b'x'*4097]))
    def test_truncated_frame(self):
        with self.assertRaises(Unsafe): self.call(self.Socket([b'{']))
    def test_multiple_frames(self):
        with self.assertRaises(Unsafe): self.call(self.Socket([b'{}\n{}\n']))
    def test_trickle_cannot_extend_deadline(self):
        with self.assertRaises(Unsafe): self.call(self.Socket([b'{',b'}\n']),clock=[0,1,2.1])

if __name__=='__main__': unittest.main()
