#!/usr/bin/env python3
"""Installed Linux control probe; never imports or invokes the real Supervisor."""
import argparse
import json
import os
import sys
from idle_stop import Controller, Transport, require

class ProbeSupervisor:
    """Caller-supplied fixture PIDs: this is not systemd ownership proof."""
    def __init__(self, pids):
        self.pids = pids
        self.inactive = set()
        self.events = []
    def state(self, service):
        os.kill(self.pids[service], 0)  # Liveness only; no signal sent.
        if service in self.inactive:
            return (0, 'inactive', 'dead', 'probe-'+service)
        return (self.pids[service], 'active', 'running', 'probe-'+service)
    def stop(self, service):
        self.events.append(['simulated_stop', service])
        self.inactive.add(service)
    def poweroff(self):
        self.events.append(['simulated_poweroff'])

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for service in ('website', 'gateway'):
        parser.add_argument('--'+service+'-pid', type=int, required=True)
        parser.add_argument('--'+service+'-revision', required=True)
    parser.add_argument('--exercise-drain-on-isolated-fixture', action='store_true')
    args = parser.parse_args()
    require(sys.platform.startswith('linux'), 'Linux required for real SO_PEERCRED')
    pids = {s: getattr(args, s+'_pid') for s in ('website','gateway')}
    require(all(pid > 1 for pid in pids.values()), 'explicit live fixture PIDs required')
    revisions = {s: getattr(args,s+'_revision') for s in pids}
    supervisor = ProbeSupervisor(pids)
    controller = Controller(supervisor, Transport(), revisions)
    try:
        # Initial status exercises real Linux peer credentials and exact candidate
        # responses without requiring the fixture to have idled 30 minutes.
        for service in pids:
            controller.control(service, 'status')
        if args.exercise_drain_on_isolated_fixture:
            controller.run(True)
    finally:
        # Simulated stop never ended a process. Restore observation and release
        # our exact owned attempts even after simulated successful poweroff.
        supervisor.inactive.clear()
        failures = []
        for service in reversed(controller.owned):
            try:
                state = controller.control(service, 'status')
                if state.get('attempt') == controller.attempt:
                    reply = controller.control(service, 'resume')
                    require(not reply['draining'] and reply['attempt'] is None, 'fixture resume failed')
            except Exception:
                failures.append(service)
        require(not failures, 'fixture drain recovery requires operator inspection')
    print(json.dumps({'status':'passed','events':supervisor.events,
                      'scope':'real_control_transport_fake_supervisor'}))

if __name__=='__main__':
    try:
        main()
    except Exception:
        print(json.dumps({'status':'failed','scope':'real_control_transport_fake_supervisor'}))
        sys.exit(1)
