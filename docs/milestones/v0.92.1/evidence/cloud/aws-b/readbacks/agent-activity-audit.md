# CloudWatch and CloudTrail attribution

- issue: #485
- profile: agent-logic-admin
- region: us-east-1
- generated_at_utc: 2026-08-27T19:29:06Z
- posture: read-only evidence collection

## CloudTrail trails

```text
{
    "trailList": []
}
```

## Recent CloudTrail events

```text
{
    "Events": [
        {
            "EventId": "01dda5ce-9835-400b-af05-277c75d19220",
            "EventName": "GetCallerIdentity",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:09:53-07:00",
            "EventSource": "sts.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\"},\"eventTime\":\"2026-08-27T19:09:53Z\",\"eventSource\":\"sts.amazonaws.com\",\"eventName\":\"GetCallerIdentity\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/AP,n,E,b,Z cfg/retry-mode#standard md/installer#source md/prompt#off md/command#sts.get-caller-identity\",\"requestParameters\":null,\"responseElements\":null,\"additionalEventData\":{\"ExtendedRequestId\":\"MTp1cy1lYXN0LTE6UzoxNzg3ODU3NzkzMTk2OlI6ZFFaY2I2azE=\",\"RequestDetails\":{\"awsServingRegion\":\"us-east-1\",\"endpointType\":\"regional\"}},\"requestID\":\"9f70fa28-cd7a-44ec-9764-bdedcc759785\",\"eventID\":\"01dda5ce-9835-400b-af05-277c75d19220\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"sts.us-east-1.amazonaws.com\",\"keyExchange\":\"X25519MLKEM768\"}}"
        },
        {
            "EventId": "e78461b5-3c7b-4d3f-ba6a-43e26a042d5d",
            "EventName": "GetCallerIdentity",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:06:49-07:00",
            "EventSource": "sts.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\"},\"eventTime\":\"2026-08-27T19:06:49Z\",\"eventSource\":\"sts.amazonaws.com\",\"eventName\":\"GetCallerIdentity\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/n,E,b,Z,AP cfg/retry-mode#standard md/installer#source md/prompt#off md/command#sts.get-caller-identity\",\"requestParameters\":null,\"responseElements\":null,\"additionalEventData\":{\"ExtendedRequestId\":\"MTp1cy1lYXN0LTE6UzoxNzg3ODU3NjA5Njk2OlI6cmI5SGk1VEM=\",\"RequestDetails\":{\"awsServingRegion\":\"us-east-1\",\"endpointType\":\"regional\"}},\"requestID\":\"27506e2f-6cd0-46b9-b4a7-cce2c5790ab3\",\"eventID\":\"e78461b5-3c7b-4d3f-ba6a-43e26a042d5d\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"sts.us-east-1.amazonaws.com\",\"keyExchange\":\"X25519MLKEM768\"}}"
        },
        {
            "EventId": "2bb20f48-963c-3dc0-a1f8-9ace782409d7",
            "EventName": "Decrypt",
            "ReadOnly": "true",
            "EventTime": "2026-08-27T12:05:57-07:00",
            "EventSource": "kms.amazonaws.com",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AWSService\",\"invokedBy\":\"cloudfront.amazonaws.com\"},\"eventTime\":\"2026-08-27T19:05:57Z\",\"eventSource\":\"kms.amazonaws.com\",\"eventName\":\"Decrypt\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"cloudfront.amazonaws.com\",\"userAgent\":\"cloudfront.amazonaws.com\",\"requestParameters\":{\"encryptionAlgorithm\":\"SYMMETRIC_DEFAULT\",\"encryptionContext\":{\"aws:acm:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:certificate/387dacdd-c6ee-43e1-9ab4-c2a2e7c250a1\",\"aws:cloudfront:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:distribution/E33B60VD3JG6BI\"}},\"responseElements\":null,\"additionalEventData\":{\"keyMaterialId\":\"934523e09eada08ef74701e680204f080ee68c3670b928ced038429a96a2edfe\"},\"requestID\":\"1bea0124-6c6b-43f9-94da-9fd44208c928\",\"eventID\":\"2bb20f48-963c-3dc0-a1f8-9ace782409d7\",\"readOnly\":true,\"resources\":[{\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"type\":\"AWS::KMS::Key\",\"ARN\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:key/dea223e2-ef4b-4f84-a952-e5ef39cb125d\"}],\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"sharedEventID\":\"f6dba4fc-4225-4e01-bb13-da274bfaa6fe\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "6eed181e-ef5b-46bd-b440-b46b35d4b680",
            "EventName": "DescribeInstances",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T11:58:03-07:00",
            "EventSource": "ec2.amazonaws.com",
            "Username": "adl-ec2-instance-age-12h-alert",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:adl-ec2-instance-age-12h-alert\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/adl-ec2-instance-age-alert-role/adl-ec2-instance-age-12h-alert\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/adl-ec2-instance-age-alert-role\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"adl-ec2-instance-age-alert-role\"},\"attributes\":{\"creationDate\":\"2026-08-27T18:57:48Z\",\"mfaAuthenticated\":\"false\"}},\"inScopeOf\":{\"issuerType\":\"AWS::Lambda::Function\",\"credentialsIssuedTo\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:function:adl-ec2-instance-age-12h-alert\",\"credentialsIssuedToVersion\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:function:adl-ec2-instance-age-12h-alert:$LATEST\"}},\"eventTime\":\"2026-08-27T18:58:03Z\",\"eventSource\":\"ec2.amazonaws.com\",\"eventName\":\"DescribeInstances\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"35.88.27.26\",\"userAgent\":\"Boto3/1.42.97 md/Botocore#1.42.97 ua/2.1 os/linux#5.10.255-262-303.1063.amzn2.x86_64 md/arch#x86_64 lang/python#3.12.13 md/pyimpl#CPython exec-env/AWS_Lambda_python3.12 m/Z,b,D,C cfg/retry-mode#legacy Botocore/1.42.97\",\"requestParameters\":{\"instancesSet\":{},\"filterSet\":{\"items\":[{\"name\":\"instance-state-name\",\"valueSet\":{\"items\":[{\"value\":\"running\"}]}}]}},\"responseElements\":null,\"requestID\":\"0820998d-fb6d-4991-bbc0-81359466e96f\",\"eventID\":\"6eed181e-ef5b-46bd-b440-b46b35d4b680\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"ec2.us-east-1.amazonaws.com\"}}"
        },
        {
            "EventId": "d6f5ad90-33c0-3ff0-801c-39bdbe254e58",
            "EventName": "Decrypt",
            "ReadOnly": "true",
            "EventTime": "2026-08-27T11:26:16-07:00",
            "EventSource": "kms.amazonaws.com",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AWSService\",\"invokedBy\":\"cloudfront.amazonaws.com\"},\"eventTime\":\"2026-08-27T18:26:16Z\",\"eventSource\":\"kms.amazonaws.com\",\"eventName\":\"Decrypt\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"cloudfront.amazonaws.com\",\"userAgent\":\"cloudfront.amazonaws.com\",\"requestParameters\":{\"encryptionAlgorithm\":\"SYMMETRIC_DEFAULT\",\"encryptionContext\":{\"aws:acm:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:certificate/ef1ec35d-59b2-4d9e-98ee-372bb4d548c2\",\"aws:cloudfront:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:distribution/E1QMUEXPA12TDK\"}},\"responseElements\":null,\"additionalEventData\":{\"keyMaterialId\":\"934523e09eada08ef74701e680204f080ee68c3670b928ced038429a96a2edfe\"},\"requestID\":\"762e45f2-b256-43d9-b6f1-ecfef56da12c\",\"eventID\":\"d6f5ad90-33c0-3ff0-801c-39bdbe254e58\",\"readOnly\":true,\"resources\":[{\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"type\":\"AWS::KMS::Key\",\"ARN\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:key/dea223e2-ef4b-4f84-a952-e5ef39cb125d\"}],\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"sharedEventID\":\"047ba4b8-7982-4dbc-b538-d5f37721707f\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "bfb995dc-163f-3565-87a4-fafde00c9a05",
            "EventName": "Decrypt",
            "ReadOnly": "true",
            "EventTime": "2026-08-27T11:06:52-07:00",
            "EventSource": "kms.amazonaws.com",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AWSService\",\"invokedBy\":\"cloudfront.amazonaws.com\"},\"eventTime\":\"2026-08-27T18:06:52Z\",\"eventSource\":\"kms.amazonaws.com\",\"eventName\":\"Decrypt\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"cloudfront.amazonaws.com\",\"userAgent\":\"cloudfront.amazonaws.com\",\"requestParameters\":{\"encryptionAlgorithm\":\"SYMMETRIC_DEFAULT\",\"encryptionContext\":{\"aws:acm:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:certificate/387dacdd-c6ee-43e1-9ab4-c2a2e7c250a1\",\"aws:cloudfront:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:distribution/E33B60VD3JG6BI\"}},\"responseElements\":null,\"additionalEventData\":{\"keyMaterialId\":\"934523e09eada08ef74701e680204f080ee68c3670b928ced038429a96a2edfe\"},\"requestID\":\"5b66723e-c279-4e59-8a6c-58a03d882f49\",\"eventID\":\"bfb995dc-163f-3565-87a4-fafde00c9a05\",\"readOnly\":true,\"resources\":[{\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"type\":\"AWS::KMS::Key\",\"ARN\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:key/dea223e2-ef4b-4f84-a952-e5ef39cb125d\"}],\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"sharedEventID\":\"2f36e450-383e-4914-ad11-56699449d1c8\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "a00300d9-d3bc-4b84-9a25-634efa8bd4be",
            "EventName": "DescribeInstances",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T10:58:03-07:00",
            "EventSource": "ec2.amazonaws.com",
            "Username": "adl-ec2-instance-age-12h-alert",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:adl-ec2-instance-age-12h-alert\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/adl-ec2-instance-age-alert-role/adl-ec2-instance-age-12h-alert\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/adl-ec2-instance-age-alert-role\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"adl-ec2-instance-age-alert-role\"},\"attributes\":{\"creationDate\":\"2026-08-27T17:57:48Z\",\"mfaAuthenticated\":\"false\"}},\"inScopeOf\":{\"issuerType\":\"AWS::Lambda::Function\",\"credentialsIssuedTo\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:function:adl-ec2-instance-age-12h-alert\",\"credentialsIssuedToVersion\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:function:adl-ec2-instance-age-12h-alert:$LATEST\"}},\"eventTime\":\"2026-08-27T17:58:03Z\",\"eventSource\":\"ec2.amazonaws.com\",\"eventName\":\"DescribeInstances\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"18.246.11.77\",\"userAgent\":\"Boto3/1.42.97 md/Botocore#1.42.97 ua/2.1 os/linux#5.10.255-262-303.1063.amzn2.x86_64 md/arch#x86_64 lang/python#3.12.13 md/pyimpl#CPython exec-env/AWS_Lambda_python3.12 m/b,D,Z,C cfg/retry-mode#legacy Botocore/1.42.97\",\"requestParameters\":{\"instancesSet\":{},\"filterSet\":{\"items\":[{\"name\":\"instance-state-name\",\"valueSet\":{\"items\":[{\"value\":\"running\"}]}}]}},\"responseElements\":null,\"requestID\":\"56aa5a4a-1aad-4bab-ad72-450df93bbe76\",\"eventID\":\"a00300d9-d3bc-4b84-9a25-634efa8bd4be\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"ec2.us-east-1.amazonaws.com\"}}"
        },
        {
            "EventId": "355690b8-3758-3a85-9ce8-422f3544fce8",
            "EventName": "Decrypt",
            "ReadOnly": "true",
            "EventTime": "2026-08-27T10:42:18-07:00",
            "EventSource": "kms.amazonaws.com",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AWSService\",\"invokedBy\":\"cloudfront.amazonaws.com\"},\"eventTime\":\"2026-08-27T17:42:18Z\",\"eventSource\":\"kms.amazonaws.com\",\"eventName\":\"Decrypt\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"cloudfront.amazonaws.com\",\"userAgent\":\"cloudfront.amazonaws.com\",\"requestParameters\":{\"encryptionContext\":{\"aws:acm:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:certificate/387dacdd-c6ee-43e1-9ab4-c2a2e7c250a1\",\"aws:cloudfront:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:distribution/E33B60VD3JG6BI\"},\"encryptionAlgorithm\":\"SYMMETRIC_DEFAULT\"},\"responseElements\":null,\"additionalEventData\":{\"keyMaterialId\":\"934523e09eada08ef74701e680204f080ee68c3670b928ced038429a96a2edfe\"},\"requestID\":\"68f2ae75-8124-4f2f-aa6e-7819abf24dc2\",\"eventID\":\"355690b8-3758-3a85-9ce8-422f3544fce8\",\"readOnly\":true,\"resources\":[{\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"type\":\"AWS::KMS::Key\",\"ARN\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:key/dea223e2-ef4b-4f84-a952-e5ef39cb125d\"}],\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"sharedEventID\":\"d95dd67b-f56f-46e7-910e-4bb5c405c088\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "a97df02f-8d40-4e6b-ad80-41a837672008",
            "EventName": "DescribeDBClusterSnapshots",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T10:42:09-07:00",
            "EventSource": "rds.amazonaws.com",
            "Username": "resource-explorer-2",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"AWSServiceRoleForResourceExplorer\"},\"attributes\":{\"creationDate\":\"2026-08-27T17:42:09Z\",\"mfaAuthenticated\":\"false\"}},\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-27T17:42:09Z\",\"eventSource\":\"rds.amazonaws.com\",\"eventName\":\"DescribeDBClusterSnapshots\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"includeShared\":false,\"includePublic\":false},\"responseElements\":null,\"requestID\":\"40ebe63f-0830-435f-8474-bfdce46fc1a6\",\"eventID\":\"a97df02f-8d40-4e6b-ad80-41a837672008\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "b070a0cd-ee86-3b53-bf04-9794f977fda7",
            "EventName": "AssumeRole",
            "ReadOnly": "true",
            "EventTime": "2026-08-27T10:42:09-07:00",
            "EventSource": "sts.amazonaws.com",
            "Resources": [
                {
                    "ResourceType": "AWS::IAM::AccessKey",
                    "ResourceName": "[AWS_IDENTIFIER_REDACTED]"
                },
                {
                    "ResourceType": "AWS::STS::AssumedRole",
                    "ResourceName": "resource-explorer-2"
                },
                {
                    "ResourceType": "AWS::STS::AssumedRole",
                    "ResourceName": "[AWS_IDENTIFIER_REDACTED]:resource-explorer-2"
                },
                {
                    "ResourceType": "AWS::STS::AssumedRole",
                    "ResourceName": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2"
                },
                {
                    "ResourceType": "AWS::IAM::Role",
                    "ResourceName": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer"
                }
            ],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AWSService\",\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-27T17:42:09Z\",\"eventSource\":\"sts.amazonaws.com\",\"eventName\":\"AssumeRole\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"roleArn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"roleSessionName\":\"resource-explorer-2\"},\"responseElements\":{\"credentials\":{\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionToken\":\"IQoJb3JpZ2luX2VjEHIaCXVzLWVhc3QtMSJGMEQCIGIEVewLCaEr9QO6MxDnjdR1sYB5xYGwBkMKAW+LnYHBAiAhDuWiKft/uXSTUIzbO8qbjAR43E+QyKx8Py0oOmDlwyryAgg7EAAaDDcxMzMzMjUyNTg4OSIMaRJt/b1bkoDU6eR/Ks8CYESeiJREzPTUmGD5ABR/YqrdKUSIxMMu+W/5TNzDMlWIjPD43MzJ+8Ad/xkpvu4w4J6qf0TfYqnWbPgOVU9T9FkpgOGfSGlx33fRO9UbU6egempdRL+fTTzp/TgdgKnT3wd/AuDhQ5GvbQxqzr0EZ+UwPVxX5QHagq7Kgau2Oyzp2s1+E3Ub9MWQPINOyF9z0lAYKLggPSGoQAuh+txe2FcJEhvCKvDsaF0wDIX9vKldGTjATInDrWxwAiry6JmatL+AOIsIx4hej4hnMkVc8jYp7ICYA1IDVWqBEuOJwvnqUcI9fvWS2Z7HYQE4gjzGa9WBASPq+kNr8zcN/nmlsfWtyCgTMe10f5f37FrwqRQrY+CUJWRXtyjDkn8nWu04yzTI9s8RRN9EQLHPqFggDEc1en6BUaSylOZGr+dKoJpi1BE0wACjx1dquI/cU1Qw8e3B1AY6vgHcQGuqKQ7lIkxUx3Fp/hSU1JMGiQRChVaghCo2pWgFhJW/ZeeNWaw/tJAcnHqwB0ENv9Bvv5wJ2QueNJOed6cHOsJrFFHMbt0CMHopvi9HgwdjXmkwQkT7l4E/RM/74JUj6WuCqjekCSFGuwwFcGxN42p4zVd8iKRcl0xxe8E9Q1H8mCFZMcN29BcgtH9ntf4YimBYdsrMtaug4d+Gd0nR+DdS/EhcJ47lbWFF9J5xVxhZUw5YYuzCsIJiQbDG\",\"expiration\":\"2026-08-27T18:42:09Z\"},\"assumedRoleUser\":{\"assumedRoleId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\"}},\"additionalEventData\":{\"ExtendedRequestId\":\"MTp1cy1lYXN0LTE6UzoxNzg3ODUyNTI5MjU3OlI6dUZhdHBrMkc=\"},\"requestID\":\"8a7c8525-2543-40e4-a990-9f9e6cbad041\",\"eventID\":\"b070a0cd-ee86-3b53-bf04-9794f977fda7\",\"readOnly\":true,\"resources\":[{\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"type\":\"AWS::IAM::Role\",\"ARN\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\"}],\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"sharedEventID\":\"e4c10f21-880a-4029-8744-8d2d5a97f961\",\"eventCategory\":\"Management\"}"
        }
    ],
    "NextToken": "E07PYNmxQ1AkA1WN5Hv+M61FTHlVFoID9s7zcO7NDYMjv74n1VD4mDovpeEpyZnV"
}
```

## CloudWatch AWS Usage metrics

```text
{
    "Metrics": [
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "AWSServiceRoleForElasticLoadBalancing"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29322484555-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4921-adl-wp07-4921-spot-20260706"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]43"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "ListCostAllocationTags"
                },
                {
                    "Name": "Service",
                    "Value": "Cost Explorer"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]42"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]07"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "adl-ebs-unattached-age-alert-role"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "rtb-0ed8b52684e5c2f9e"
                },
                {
                    "Name": "Resource",
                    "Value": "RoutesPerRouteTable"
                },
                {
                    "Name": "Service",
                    "Value": "EC2"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLWP5795GpuDeadlineReaperRole"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-lifecycle-222f"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "adl-csm-notice-receiver-4998-role"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "adl-csm-notice-receiver-4998-role"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DescribeTrails"
                },
                {
                    "Name": "Service",
                    "Value": "CloudTrail"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-79262133a739"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]30"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260819-22"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLWP5795GpuDeadlineReaperRole"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]19"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]19"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "116e3bd6-ed4b-4ad7-94d2-0ecf6cf42440"
                },
                {
                    "Name": "Resource",
                    "Value": "AliasesPerKey"
                },
                {
                    "Name": "Service",
                    "Value": "KMS"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "codefriend-ai-github-deploy"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29632596016-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "User"
                },
                {
                    "Name": "Service",
                    "Value": "ElastiCache"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260819-22"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-lifecycle-4a38"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]08"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "adl-csm-5039-api-gateway-bridge-role"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLRemoteValidationPermanentRole"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "scr-archive-relay-role"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "AWSServiceRoleForAPIGateway"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29477416287-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]08"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "OptOutLists"
                },
                {
                    "Name": "Service",
                    "Value": "End User Messaging SMS"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29632596016-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29470867446-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "OIDCProvider"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "AWSServiceRoleForServiceQuotas"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-0c3d5e093e"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "adl-spot-remote-validation-github-actions-role"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "agent-logic-ai-github-deploy"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "GetCostAndUsage"
                },
                {
                    "Name": "Service",
                    "Value": "Cost Explorer"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "scr-archive-relay-role"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]42"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "ListMetrics"
                },
                {
                    "Name": "Service",
                    "Value": "CloudWatch"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29470867446-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5574-adl-wp-5574-gha-29700301599-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29191264648-1"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-0c3d5e093e"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]41"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]07"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLHybridSSMManagedNodeRole"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29322484555-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "GetAnomalyMonitors"
                },
                {
                    "Name": "Service",
                    "Value": "Cost Explorer"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4921-adl-wp07-4921-spot-20260706"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]53"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260820-38"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "User"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29321998194-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-lifecycle-222f"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]43"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DescribeBudgets"
                },
                {
                    "Name": "Service",
                    "Value": "Budgets"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "WebAclsPerAccountCloudFront"
                },
                {
                    "Name": "Service",
                    "Value": "AWS WAF"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "GeneralPurposeBuckets"
                },
                {
                    "Name": "Service",
                    "Value": "S3"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "dea223e2-ef4b-4f84-a952-e5ef39cb125d"
                },
                {
                    "Name": "Resource",
                    "Value": "GrantsPerKey"
                },
                {
                    "Name": "Service",
                    "Value": "KMS"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "dea223e2-ef4b-4f84-a952-e5ef39cb125d"
                },
                {
                    "Name": "Resource",
                    "Value": "AliasesPerKey"
                },
                {
                    "Name": "Service",
                    "Value": "KMS"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29477416287-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "AWSServiceRoleForResourceExplorer"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DescribeDBClusterSnapshots"
                },
                {
                    "Name": "Service",
                    "Value": "RDS"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29191264648-1"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "DomainCount"
                },
                {
                    "Name": "Service",
                    "Value": "Route 53 Domains"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "adl-codefriend-codebuild-service-role"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "daniel.austin.admin"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerUser"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260820-38"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "sg-0220ad0c1b60b7cf9"
                },
                {
                    "Name": "Resource",
                    "Value": "RulesPerSecurityGroup"
                },
                {
                    "Name": "Service",
                    "Value": "EC2"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "vpc-080be4051e99f513b"
                },
                {
                    "Name": "Resource",
                    "Value": "VPCsPerRegion"
                },
                {
                    "Name": "Service",
                    "Value": "EC2"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DescribeInstances"
                },
                {
                    "Name": "Service",
                    "Value": "EC2"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-79262133a739"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "adl-ec2-instance-age-alert-role"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "AWSServiceRoleForEC2Spot"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]53"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "adl-codefriend-github-actions-build-role"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-lifecycle-4a38"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "LookupEvents"
                },
                {
                    "Name": "Service",
                    "Value": "CloudTrail"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29321998194-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "KeysPerAccount"
                },
                {
                    "Name": "Service",
                    "Value": "KMS"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]41"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLHybridSSMManagedNodeRole"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-5574-adl-wp-5574-gha-29700301599-1-ad"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]30"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "Role"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260819-27"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "InstanceProfile"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260819-27"
                },
                {
                    "Name": "Resource",
                    "Value": "ManagedPoliciesPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        },
        {
            "Namespace": "AWS/Usage",
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "ResourceId",
                    "Value": "ADLRemoteValidationPermanentRole"
                },
                {
                    "Name": "Resource",
                    "Value": "TrustPolicyLengthPerRole"
                },
                {
                    "Name": "Service",
                    "Value": "IAM"
                },
                {
                    "Name": "Class",
                    "Value": "None"
                }
            ]
        }
    ]
}
```

