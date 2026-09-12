#!/usr/bin/env python3
"""PVF: deterministic local CPU; required issue gate, not release-wide proof."""
import copy
import datetime as dt
import json
import unittest
import inventory as i

class Negatives(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet=json.loads((i.HERE/'inventory.json').read_text())
        cls.reference=dt.datetime.fromisoformat(cls.packet['completed_at'])
    def check(self,p): return i.validate(p,self.reference)
    def test_real_packet(self): self.check(self.packet)
    def test_wrong_account(self):
        p=copy.deepcopy(self.packet);p['identity']['verified_business_account']=False
        with self.assertRaisesRegex(AssertionError,'wrong account'):self.check(p)
    def test_missing_surface(self):
        p=copy.deepcopy(self.packet);p['surfaces'].pop()
        with self.assertRaisesRegex(AssertionError,'missing or duplicate'):self.check(p)
    def test_stale_evidence(self):
        p=copy.deepcopy(self.packet);p['identity']['captured_at']='2000-01-01T00:00:00+00:00'
        with self.assertRaisesRegex(AssertionError,'stale'):self.check(p)
    def test_raw_identifier(self):
        p=copy.deepcopy(self.packet);p['unsafe']='arn:aws:iam::123456789012:user/example'
        with self.assertRaisesRegex(AssertionError,'redaction'):self.check(p)
    def test_missing_bucket_metadata(self):
        p=copy.deepcopy(self.packet);p['buckets'].pop()
        with self.assertRaisesRegex(AssertionError,'missing bucket'):self.check(p)
    def test_identity_mismatch(self):
        self.assertFalse(i.identity_ok({'Account':'111111111111','Arn':'arn:aws:iam::111111111111:user/example'},'222222222222'))
    def test_missing_response_is_not_empty(self):
        with self.assertRaises(ValueError): i.project({},'s3-buckets',i.GLOBAL['s3-buckets'])
    def test_missing_cloudfront_items(self):
        with self.assertRaises(ValueError): i.project({'DistributionList':{}},'cloudfront-distributions',i.GLOBAL['cloudfront-distributions'])
    def test_numeric_identifier_rejected(self):
        p=copy.deepcopy(self.packet);p['Account']=123456789012
        with self.assertRaisesRegex(AssertionError,'unexpected numeric'):self.check(p)
    def test_delta_tamper(self):
        p=copy.deepcopy(self.packet);p['delta'][0]['baseline_count']=999
        with self.assertRaisesRegex(AssertionError,'delta mismatch'):self.check(p)
    def test_read_failure_not_absence(self):
        p=copy.deepcopy(self.packet);p['surfaces'][0]['status']='read-failed'
        with self.assertRaisesRegex(AssertionError,'failure conflated'):self.check(p)
    def test_mutation_rejected(self):
        with self.assertRaises(AssertionError):i.aws('ec2','terminate-instances')

if __name__=='__main__':unittest.main()
