# Billing budget anomaly export and cost attribution

- issue: #485
- profile: agent-logic-admin
- region: us-east-1
- generated_at_utc: 2026-08-27T19:40:28Z
- posture: read-only evidence collection

## Cost Explorer seven-day unblended cost

```text
{
    "ResultsByTime": [
        {
            "TimePeriod": {
                "Start": "2026-08-20",
                "End": "2026-08-21"
            },
            "Total": {
                "UnblendedCost": {
                    "Amount": "4.1294931175",
                    "Unit": "USD"
                }
            },
            "Groups": [],
            "Estimated": true
        },
        {
            "TimePeriod": {
                "Start": "2026-08-21",
                "End": "2026-08-22"
            },
            "Total": {
                "UnblendedCost": {
                    "Amount": "5.7024719901",
                    "Unit": "USD"
                }
            },
            "Groups": [],
            "Estimated": true
        },
        {
            "TimePeriod": {
                "Start": "2026-08-22",
                "End": "2026-08-23"
            },
            "Total": {
                "UnblendedCost": {
                    "Amount": "13.7492572494",
                    "Unit": "USD"
                }
            },
            "Groups": [],
            "Estimated": true
        },
        {
            "TimePeriod": {
                "Start": "2026-08-23",
                "End": "2026-08-24"
            },
            "Total": {
                "UnblendedCost": {
                    "Amount": "6.6625606921",
                    "Unit": "USD"
                }
            },
            "Groups": [],
            "Estimated": true
        },
        {
            "TimePeriod": {
                "Start": "2026-08-24",
                "End": "2026-08-25"
            },
            "Total": {
                "UnblendedCost": {
                    "Amount": "6.652566095",
                    "Unit": "USD"
                }
            },
            "Groups": [],
            "Estimated": true
        },
        {
            "TimePeriod": {
                "Start": "2026-08-25",
                "End": "2026-08-26"
            },
            "Total": {
                "UnblendedCost": {
                    "Amount": "4.6289413666",
                    "Unit": "USD"
                }
            },
            "Groups": [],
            "Estimated": true
        },
        {
            "TimePeriod": {
                "Start": "2026-08-26",
                "End": "2026-08-27"
            },
            "Total": {
                "UnblendedCost": {
                    "Amount": "0.320200436",
                    "Unit": "USD"
                }
            },
            "Groups": [],
            "Estimated": true
        }
    ],
    "DimensionValueAttributes": []
}
```

## Budgets

```text
{
    "Budgets": [
        {
            "BudgetName": "Agent Logic Monthly",
            "BudgetLimit": {
                "Amount": "500.0",
                "Unit": "USD"
            },
            "TimeUnit": "MONTHLY",
            "TimePeriod": {
                "Start": "2026-05-31T17:00:00-07:00",
                "End": "2087-06-14T17:00:00-07:00"
            },
            "CalculatedSpend": {
                "ActualSpend": {
                    "Amount": "105.107",
                    "Unit": "USD"
                },
                "ForecastedSpend": {
                    "Amount": "133.053",
                    "Unit": "USD"
                }
            },
            "BudgetType": "COST",
            "LastUpdatedTime": "2026-08-27T11:54:09.932000-07:00",
            "FilterExpression": {
                "Not": {
                    "Dimensions": {
                        "Key": "RECORD_TYPE",
                        "Values": [
                            "Credit",
                            "Refund"
                        ]
                    }
                }
            },
            "Metrics": [
                "UnblendedCost"
            ],
            "BillingViewArn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:billingview/primary",
            "HealthStatus": {
                "Status": "HEALTHY",
                "LastUpdatedTime": "2026-08-27T11:54:09.476000-07:00"
            }
        }
    ]
}
```

## Cost anomaly monitors

```text
{
    "AnomalyMonitors": [
        {
            "MonitorArn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:anomalymonitor/4e5ff3a0-9489-40c3-a0d9-159ce49e2770",
            "MonitorName": "Default-Services-Monitor",
            "CreationDate": "2026-06-20T01:02:55.750643086Z",
            "LastUpdatedDate": "2026-08-27T12:47:45.248Z",
            "LastEvaluatedDate": "2026-08-27T12:47:45.248632188Z",
            "MonitorType": "DIMENSIONAL",
            "MonitorDimension": "SERVICE",
            "DimensionalValueCount": 18
        }
    ]
}
```

## Cost allocation tags

```text
{
    "CostAllocationTags": [
        {
            "TagKey": "ADLProbe",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-06-01T00:00:00Z"
        },
        {
            "TagKey": "Account",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Component",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "CsmName",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Environment",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Host",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "HostRole",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Issue",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Lane",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "ManagedBy",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Name",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Project",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Purpose",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Stack",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "Teardown",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-06-01T00:00:00Z"
        },
        {
            "TagKey": "adl:cleanup_required",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:component",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:delete_after",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:hosted_model_fallback",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:issue",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:lane",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-07-01T00:00:00Z"
        },
        {
            "TagKey": "adl:managed",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-07-01T00:00:00Z"
        },
        {
            "TagKey": "adl:milestone",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:node_id",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:owner",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-07-01T00:00:00Z"
        },
        {
            "TagKey": "adl:public_runtime_exposure",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:purchase_option",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:purpose",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:retained",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:retention_reason",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:role",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:run-id",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-07-01T00:00:00Z"
        },
        {
            "TagKey": "adl:run_id",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:snapshot-generation",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:source_run",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:source_volume",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:ttl_expires_at",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "adl:volume_role",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "aws:cloudformation:logical-id",
            "Type": "AWSGenerated",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "aws:cloudformation:stack-id",
            "Type": "AWSGenerated",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "aws:cloudformation:stack-name",
            "Type": "AWSGenerated",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "aws:createdBy",
            "Type": "AWSGenerated",
            "Status": "Inactive",
            "LastUsedDate": "2026-08-01T00:00:00Z"
        },
        {
            "TagKey": "iamPrincipal/[AWS_IDENTIFIER_REDACTED]",
            "Type": "UserDefined",
            "Status": "Inactive",
            "LastUsedDate": "2026-07-01T00:00:00Z"
        }
    ]
}
```

## Billing exports

```text
{
    "Exports": []
}
```

