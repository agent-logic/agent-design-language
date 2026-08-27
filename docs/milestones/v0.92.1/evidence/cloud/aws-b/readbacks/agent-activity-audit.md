# CloudWatch and CloudTrail attribution

- issue: #485
- profile: agent-logic-admin
- region: us-east-1
- generated_at_utc: 2026-08-26T20:44:14Z
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
            "EventId": "ff09c6a7-dd45-313d-b321-72ad2cf4a755",
            "EventName": "Decrypt",
            "ReadOnly": "true",
            "EventTime": "2026-08-26T13:33:55-07:00",
            "EventSource": "kms.amazonaws.com",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AWSService\",\"invokedBy\":\"cloudfront.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:33:55Z\",\"eventSource\":\"kms.amazonaws.com\",\"eventName\":\"Decrypt\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"cloudfront.amazonaws.com\",\"userAgent\":\"cloudfront.amazonaws.com\",\"requestParameters\":{\"encryptionAlgorithm\":\"SYMMETRIC_DEFAULT\",\"encryptionContext\":{\"aws:acm:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:certificate/3c389c89-1f6d-4292-ae95-a006738690f0\",\"aws:cloudfront:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:distribution/E3C29FMX32KDDU\"}},\"responseElements\":null,\"additionalEventData\":{\"keyMaterialId\":\"934523e09eada08ef74701e680204f080ee68c3670b928ced038429a96a2edfe\"},\"requestID\":\"13e74504-41aa-4796-92a7-ff57a1ad8793\",\"eventID\":\"ff09c6a7-dd45-313d-b321-72ad2cf4a755\",\"readOnly\":true,\"resources\":[{\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"type\":\"AWS::KMS::Key\",\"ARN\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:key/dea223e2-ef4b-4f84-a952-e5ef39cb125d\"}],\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"sharedEventID\":\"7323222e-6d7c-4a89-8e1d-1ecb62f9f0fa\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "9fdf4255-6642-31f4-9444-f1d7c0017cbe",
            "EventName": "Decrypt",
            "ReadOnly": "true",
            "EventTime": "2026-08-26T13:24:48-07:00",
            "EventSource": "kms.amazonaws.com",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AWSService\",\"invokedBy\":\"cloudfront.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:24:48Z\",\"eventSource\":\"kms.amazonaws.com\",\"eventName\":\"Decrypt\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"cloudfront.amazonaws.com\",\"userAgent\":\"cloudfront.amazonaws.com\",\"requestParameters\":{\"encryptionContext\":{\"aws:acm:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:certificate/387dacdd-c6ee-43e1-9ab4-c2a2e7c250a1\",\"aws:cloudfront:arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:distribution/E33B60VD3JG6BI\"},\"encryptionAlgorithm\":\"SYMMETRIC_DEFAULT\"},\"responseElements\":null,\"additionalEventData\":{\"keyMaterialId\":\"934523e09eada08ef74701e680204f080ee68c3670b928ced038429a96a2edfe\"},\"requestID\":\"37a13e4b-b7c4-4032-b010-c7c7a2ba6785\",\"eventID\":\"9fdf4255-6642-31f4-9444-f1d7c0017cbe\",\"readOnly\":true,\"resources\":[{\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"type\":\"AWS::KMS::Key\",\"ARN\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:key/dea223e2-ef4b-4f84-a952-e5ef39cb125d\"}],\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"sharedEventID\":\"9afc431b-fcee-4a08-8d8c-57f5a5ba51a5\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "3e69a435-79e5-4ccb-ae13-2013919ec00c",
            "EventName": "ListTagsForResource",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-26T13:19:06-07:00",
            "EventSource": "cloudfront.amazonaws.com",
            "Username": "resource-explorer-2",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"AWSServiceRoleForResourceExplorer\"},\"attributes\":{\"creationDate\":\"2026-08-26T20:19:04Z\",\"mfaAuthenticated\":\"false\"}},\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:19:06Z\",\"eventSource\":\"cloudfront.amazonaws.com\",\"eventName\":\"ListTagsForResource\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"resource\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:distribution/E2A4Y69MBQG519\"},\"responseElements\":null,\"requestID\":\"8fa0a56c-5758-4580-bdaa-405077c2ae6a\",\"eventID\":\"3e69a435-79e5-4ccb-ae13-2013919ec00c\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"apiVersion\":\"2020_05_31\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "4b02cb54-a8f8-4c50-a672-014af35387d7",
            "EventName": "GetDistribution",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-26T13:19:06-07:00",
            "EventSource": "cloudfront.amazonaws.com",
            "Username": "resource-explorer-2",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"AWSServiceRoleForResourceExplorer\"},\"attributes\":{\"creationDate\":\"2026-08-26T20:19:04Z\",\"mfaAuthenticated\":\"false\"}},\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:19:06Z\",\"eventSource\":\"cloudfront.amazonaws.com\",\"eventName\":\"GetDistribution\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"id\":\"E2A4Y69MBQG519\"},\"responseElements\":null,\"requestID\":\"319924eb-dfb4-48b6-b357-5635dcabffab\",\"eventID\":\"4b02cb54-a8f8-4c50-a672-014af35387d7\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"apiVersion\":\"2020_05_31\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "51819259-dbaf-4c85-a220-9363e8262902",
            "EventName": "GetDistribution",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-26T13:19:06-07:00",
            "EventSource": "cloudfront.amazonaws.com",
            "Username": "resource-explorer-2",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"AWSServiceRoleForResourceExplorer\"},\"attributes\":{\"creationDate\":\"2026-08-26T20:19:04Z\",\"mfaAuthenticated\":\"false\"}},\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:19:06Z\",\"eventSource\":\"cloudfront.amazonaws.com\",\"eventName\":\"GetDistribution\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"id\":\"E2P8CMPYZNLKVX\"},\"responseElements\":null,\"requestID\":\"303d5dc8-d848-436f-a126-a4130434b112\",\"eventID\":\"51819259-dbaf-4c85-a220-9363e8262902\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"apiVersion\":\"2020_05_31\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "ae05bcdd-7341-4f54-bd70-69c56b3ef579",
            "EventName": "ListTagsForResource",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-26T13:19:06-07:00",
            "EventSource": "cloudfront.amazonaws.com",
            "Username": "resource-explorer-2",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"AWSServiceRoleForResourceExplorer\"},\"attributes\":{\"creationDate\":\"2026-08-26T20:19:04Z\",\"mfaAuthenticated\":\"false\"}},\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:19:06Z\",\"eventSource\":\"cloudfront.amazonaws.com\",\"eventName\":\"ListTagsForResource\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"resource\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:distribution/E2P8CMPYZNLKVX\"},\"responseElements\":null,\"requestID\":\"e9cfc6ac-ee0f-44e2-ab37-b6f1579a0f45\",\"eventID\":\"ae05bcdd-7341-4f54-bd70-69c56b3ef579\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"apiVersion\":\"2020_05_31\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "ee5eacb7-ea1d-4e8a-a353-075fae88aca0",
            "EventName": "GetResource",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-26T13:19:06-07:00",
            "EventSource": "cloudcontrolapi.amazonaws.com",
            "Username": "resource-explorer-2",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"AWSServiceRoleForResourceExplorer\"},\"attributes\":{\"creationDate\":\"2026-08-26T20:19:04Z\",\"mfaAuthenticated\":\"false\"}},\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:19:06Z\",\"eventSource\":\"cloudcontrolapi.amazonaws.com\",\"eventName\":\"GetResource\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"typeName\":\"AWS::CloudFront::Distribution\",\"identifier\":\"E2A4Y69MBQG519\"},\"responseElements\":null,\"requestID\":\"e4ecd26e-d2c0-4ffa-b5a1-02c0f4545df3\",\"eventID\":\"ee5eacb7-ea1d-4e8a-a353-075fae88aca0\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "f048c175-5f69-4f3f-8899-60a077083558",
            "EventName": "GetResource",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-26T13:19:06-07:00",
            "EventSource": "cloudcontrolapi.amazonaws.com",
            "Username": "resource-explorer-2",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"AWSServiceRoleForResourceExplorer\"},\"attributes\":{\"creationDate\":\"2026-08-26T20:19:04Z\",\"mfaAuthenticated\":\"false\"}},\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:19:06Z\",\"eventSource\":\"cloudcontrolapi.amazonaws.com\",\"eventName\":\"GetResource\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"typeName\":\"AWS::CloudFront::Distribution\",\"identifier\":\"E2P8CMPYZNLKVX\"},\"responseElements\":null,\"requestID\":\"608df68f-7d7d-4f9c-a4a9-b2cf84833a0c\",\"eventID\":\"f048c175-5f69-4f3f-8899-60a077083558\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "1cd283a4-0233-4d3e-9667-73888188b762",
            "EventName": "ListTagsForResource",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-26T13:19:05-07:00",
            "EventSource": "cloudfront.amazonaws.com",
            "Username": "resource-explorer-2",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"AWSServiceRoleForResourceExplorer\"},\"attributes\":{\"creationDate\":\"2026-08-26T20:19:04Z\",\"mfaAuthenticated\":\"false\"}},\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:19:05Z\",\"eventSource\":\"cloudfront.amazonaws.com\",\"eventName\":\"ListTagsForResource\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"resource\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:distribution/E33B60VD3JG6BI\"},\"responseElements\":null,\"requestID\":\"534480d8-45cc-4ce7-ab48-cffc9a63b361\",\"eventID\":\"1cd283a4-0233-4d3e-9667-73888188b762\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"apiVersion\":\"2020_05_31\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\"}"
        },
        {
            "EventId": "880a9837-b11f-4303-90bb-dcc5d4dbeecf",
            "EventName": "GetDistribution",
            "ReadOnly": "true",
            "AccessKeyId": "[AWS_IDENTIFIER_REDACTED]",
            "EventTime": "2026-08-26T13:19:05-07:00",
            "EventSource": "cloudfront.amazonaws.com",
            "Username": "resource-explorer-2",
            "Resources": [],
            "CloudTrailEvent": "{\"eventVersion\":\"1.11\",\"userIdentity\":{\"type\":\"AssumedRole\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]:resource-explorer-2\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:assumed-role/AWSServiceRoleForResourceExplorer/resource-explorer-2\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"accessKeyId\":\"[AWS_IDENTIFIER_REDACTED]\",\"sessionContext\":{\"sessionIssuer\":{\"type\":\"Role\",\"principalId\":\"[AWS_IDENTIFIER_REDACTED]\",\"arn\":\"[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer\",\"accountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"userName\":\"AWSServiceRoleForResourceExplorer\"},\"attributes\":{\"creationDate\":\"2026-08-26T20:19:04Z\",\"mfaAuthenticated\":\"false\"}},\"invokedBy\":\"resource-explorer-2.amazonaws.com\"},\"eventTime\":\"2026-08-26T20:19:05Z\",\"eventSource\":\"cloudfront.amazonaws.com\",\"eventName\":\"GetDistribution\",\"awsRegion\":\"us-east-1\",\"sourceIPAddress\":\"resource-explorer-2.amazonaws.com\",\"userAgent\":\"resource-explorer-2.amazonaws.com\",\"requestParameters\":{\"id\":\"E33B60VD3JG6BI\"},\"responseElements\":null,\"requestID\":\"74571cbd-4d85-4ab5-bb68-211a0da5667c\",\"eventID\":\"880a9837-b11f-4303-90bb-dcc5d4dbeecf\",\"readOnly\":true,\"eventType\":\"AwsApiCall\",\"apiVersion\":\"2020_05_31\",\"managementEvent\":true,\"recipientAccountId\":\"[AWS_ACCOUNT_ID_REDACTED]\",\"eventCategory\":\"Management\"}"
        }
    ],
    "NextToken": "Nnaex2xb76b9YHiqRDtRu0WrQRAXvn4Fcc5wWXkqFWgUYdTxRm6L6ju9hSUIUdri"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "ListHostedZones"
                },
                {
                    "Name": "Service",
                    "Value": "Route 53"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DeleteCertificate"
                },
                {
                    "Name": "Service",
                    "Value": "Certificate Manager"
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
                    "Value": "ListDomains"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DescribeLoadBalancers"
                },
                {
                    "Name": "Service",
                    "Value": "Elastic Load Balancing"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "GetDomainDetail"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DescribeVolumes"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DescribeSubnets"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "ChangeResourceRecordSets"
                },
                {
                    "Name": "Service",
                    "Value": "Route 53"
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
                    "Value": "GetOperationDetail"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DescribeVpcs"
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
                    "Value": "ListHostedZonesByName"
                },
                {
                    "Name": "Service",
                    "Value": "Route 53"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "ListStacks"
                },
                {
                    "Name": "Service",
                    "Value": "CloudFormation"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "GetHostedZone"
                },
                {
                    "Name": "Service",
                    "Value": "Route 53"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "ListResourceRecordSets"
                },
                {
                    "Name": "Service",
                    "Value": "Route 53"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "DescribeSecurityGroups"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "AcceptDomainTransferFromAnotherAwsAccount"
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
            "MetricName": "CallCount",
            "Dimensions": [
                {
                    "Name": "Type",
                    "Value": "API"
                },
                {
                    "Name": "Resource",
                    "Value": "ListEmailIdentities"
                },
                {
                    "Name": "Service",
                    "Value": "SES"
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
                    "Value": "ListCertificates"
                },
                {
                    "Name": "Service",
                    "Value": "Certificate Manager"
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
                    "Value": "GetChange"
                },
                {
                    "Name": "Service",
                    "Value": "Route 53"
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
        }
    ]
}
```

