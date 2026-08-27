# CloudWatch and CloudTrail attribution

- issue: #485
- profile: agent-logic-admin
- region: us-east-1
- generated_at_utc: 2026-08-27T19:40:25Z
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
            "EventId": "42f9fb34-233e-4c7b-8e7a-89c2abb242d1",
            "EventName": "GetAccountSummary",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:30-07:00",
            "EventSource": "iam.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\"},\"eventTime\":\"2026-08-27T19:29:30Z\",\"eventSource\":\"iam.amazonaws.com\",\"eventName\":\"GetAccountSummary\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/E,AP,n,Z,b cfg/retry-mode#standard md/installer#source md/prompt#off md/command#iam.get-account-summary\",\"requestParameters\":null,\"responseElements\":null,\"requestID\":\"068672fe-5c3e-49dc-bb12-1e5106127042\",\"eventID\":\"42f9fb34-233e-4c7b-8e7a-89c2abb242d1\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"iam.amazonaws.com\"}}"
        },
        {
            "EventId": "ce35fe43-24c2-41a8-96f9-66e2828f1eb4",
            "EventName": "ListAccountAliases",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:29-07:00",
            "EventSource": "iam.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\"},\"eventTime\":\"2026-08-27T19:29:29Z\",\"eventSource\":\"iam.amazonaws.com\",\"eventName\":\"ListAccountAliases\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/b,C,n,AP,E,Z cfg/retry-mode#standard md/installer#source md/prompt#off md/command#iam.list-account-aliases\",\"requestParameters\":null,\"responseElements\":null,\"requestID\":\"aade79d2-a642-4c26-8316-794662b491a3\",\"eventID\":\"ce35fe43-24c2-41a8-96f9-66e2828f1eb4\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"iam.amazonaws.com\"}}"
        },
        {
            "EventId": "6572f618-d292-4e8c-b605-fa5974e117f3",
            "EventName": "GetCallerIdentity",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:28-07:00",
            "EventSource": "sts.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\"},\"eventTime\":\"2026-08-27T19:29:28Z\",\"eventSource\":\"sts.amazonaws.com\",\"eventName\":\"GetCallerIdentity\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/b,Z,E,AP,n cfg/retry-mode#standard md/installer#source md/prompt#off md/command#sts.get-caller-identity\",\"requestParameters\":null,\"responseElements\":null,\"additionalEventData\":{\"ExtendedRequestId\":\"MTp1cy1lYXN0LTE6UzoxNzg3ODU4OTY4NDgxOlI6MkdVdXpram4=\",\"RequestDetails\":{\"endpointType\":\"regional\",\"awsServingRegion\":\"us-east-1\"}},\"requestID\":\"787c8735-4c6e-4d3e-88eb-ed427e0634ea\",\"eventID\":\"6572f618-d292-4e8c-b605-fa5974e117f3\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"sts.us-east-1.amazonaws.com\",\"keyExchange\":\"X25519MLKEM768\"}}"
        },
        {
            "EventId": "f38594f9-9a8e-4cbf-8a1a-2384fb3b6768",
            "EventName": "ListGroups",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:27-07:00",
            "EventSource": "iam.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\"},\"eventTime\":\"2026-08-27T19:29:27Z\",\"eventSource\":\"iam.amazonaws.com\",\"eventName\":\"ListGroups\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/b,Z,C,E,n,AP cfg/retry-mode#standard md/installer#source md/prompt#off md/command#iam.list-groups\",\"requestParameters\":null,\"responseElements\":null,\"requestID\":\"4552326c-9484-4848-b9ba-dc7bdeca53c4\",\"eventID\":\"f38594f9-9a8e-4cbf-8a1a-2384fb3b6768\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"iam.amazonaws.com\"}}"
        },
        {
            "EventId": "974b778c-5fa6-4dc1-9e8e-a6c7e989f89d",
            "EventName": "ListRoles",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:26-07:00",
            "EventSource": "iam.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\"},\"eventTime\":\"2026-08-27T19:29:26Z\",\"eventSource\":\"iam.amazonaws.com\",\"eventName\":\"ListRoles\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/C,n,E,Z,AP,b cfg/retry-mode#standard md/installer#source md/prompt#off md/command#iam.list-roles\",\"requestParameters\":null,\"responseElements\":null,\"requestID\":\"7c5bf57f-9fc7-42a1-b42a-ff44c29e0b37\",\"eventID\":\"974b778c-5fa6-4dc1-9e8e-a6c7e989f89d\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"iam.amazonaws.com\"}}"
        },
        {
            "EventId": "02d47464-e626-49dd-8cfc-2b0516edba62",
            "EventName": "ListUsers",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:24-07:00",
            "EventSource": "iam.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\"},\"eventTime\":\"2026-08-27T19:29:24Z\",\"eventSource\":\"iam.amazonaws.com\",\"eventName\":\"ListUsers\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/AP,n,C,b,E,Z cfg/retry-mode#standard md/installer#source md/prompt#off md/command#iam.list-users\",\"requestParameters\":null,\"responseElements\":null,\"requestID\":\"db93d17b-d71f-4fba-9edb-993f14ba74fd\",\"eventID\":\"02d47464-e626-49dd-8cfc-2b0516edba62\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"tlsVersion\":\"TLSv1.3\",\"cipherSuite\":\"TLS_AES_128_GCM_SHA256\",\"clientProvidedHostHeader\":\"iam.amazonaws.com\"}}"
        },
        {
            "EventId": "63de74e2-9635-4d93-b3c5-3ba311a4ad69",
            "EventName": "ListExports",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:23-07:00",
            "EventSource": "bcm-data-exports.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\",\"inScopeOf\":{}},\"eventTime\":\"2026-08-27T19:29:23Z\",\"eventSource\":\"bcm-data-exports.amazonaws.com\",\"eventName\":\"ListExports\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/b,E,n,AP,C,Z cfg/retry-mode#standard md/installer#source md/prompt#off md/command#bcm-data-exports.list-exports\",\"requestParameters\":null,\"responseElements\":null,\"requestID\":\"649b64ba-e140-4129-808e-0a4e0d1a36f7\",\"eventID\":\"63de74e2-9635-4d93-b3c5-3ba311a4ad69\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"clientProvidedHostHeader\":\"bcm-data-exports.us-east-1.api.aws\"}}"
        },
        {
            "EventId": "c9ffe372-db93-4d27-9600-9bc62bfba22a",
            "EventName": "ListCostAllocationTags",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:22-07:00",
            "EventSource": "ce.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\",\"inScopeOf\":{}},\"eventTime\":\"2026-08-27T19:29:22Z\",\"eventSource\":\"ce.amazonaws.com\",\"eventName\":\"ListCostAllocationTags\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/Z,b,C,AP,n,E cfg/retry-mode#standard md/installer#source md/prompt#off md/command#ce.list-cost-allocation-tags\",\"requestParameters\":null,\"responseElements\":null,\"requestID\":\"5bd1bfd2-02bc-4754-8175-3b933db95e25\",\"eventID\":\"c9ffe372-db93-4d27-9600-9bc62bfba22a\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"clientProvidedHostHeader\":\"ce.us-east-1.amazonaws.com\"}}"
        },
        {
            "EventId": "533c385f-2c89-4a46-8db6-156b3094cf4d",
            "EventName": "GetAnomalyMonitors",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:21-07:00",
            "EventSource": "ce.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\",\"inScopeOf\":{}},\"eventTime\":\"2026-08-27T19:29:21Z\",\"eventSource\":\"ce.amazonaws.com\",\"eventName\":\"GetAnomalyMonitors\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/C,b,n,E,AP,Z cfg/retry-mode#standard md/installer#source md/prompt#off md/command#ce.get-anomaly-monitors\",\"requestParameters\":null,\"responseElements\":null,\"requestID\":\"529b7188-e1a7-4d52-9643-f3fd5c93cc5e\",\"eventID\":\"533c385f-2c89-4a46-8db6-156b3094cf4d\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"clientProvidedHostHeader\":\"ce.us-east-1.amazonaws.com\"}}"
        },
        {
            "EventId": "f8c4f5fc-6a77-406e-a955-3f55558c6c77",
            "EventName": "DescribeBudgets",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-27T12:29:19-07:00",
            "EventSource": "budgets.amazonaws.com",
            "Username": "daniel.austin.admin",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"IAMUser\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"userName\":\"daniel.austin.admin\",\"inScopeOf\":{}},\"eventTime\":\"2026-08-27T19:29:19Z\",\"eventSource\":\"budgets.amazonaws.com\",\"eventName\":\"DescribeBudgets\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"47.146.81.109\",\"userAgent\":\"aws-cli/2.36.32 md/awscrt#0.36.2 ua/2.1 os/macos#25.5.0 md/arch#arm64 lang/python#3.14.7 md/pyimpl#CPython m/AP,b,Z,E,n,C cfg/retry-mode#standard md/installer#source md/prompt#off md/command#budgets.describe-budgets\",\"requestParameters\":{\"AccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\"},\"responseElements\":null,\"requestID\":\"05a8aefe-5898-4249-a5d1-5423a39b8c6a\",\"eventID\":\"f8c4f5fc-6a77-406e-a955-3f55558c6c77\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\",\"tlsDetails\":{\"clientProvidedHostHeader\":\"budgets.amazonaws.com\"}}"
        }
    ],
    "NextToken": "ucxo6aLrjrP53+BreXWPLLQQqb9P8zVMnRqib5bhj1pHTnkrHP2angSWC4vZPCqt"
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
            "MetricName": "ResourceCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "Resource"
                },
                {
                    "Name": "Resource",
                    "Value": "Trail"
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
        }
    ]
}
```

