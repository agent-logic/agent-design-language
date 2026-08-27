# Human workload and agent identity census

- issue: #485
- profile: agent-logic-admin
- region: us-east-1
- generated_at_utc: 2026-08-26T20:44:08Z
- posture: read-only evidence collection

## Classification rule

- Humans: IAM users or roles explicitly intended for named human administration.
- Workloads: service, runtime, CI, Terraform, or deployment roles.
- Agents: Codex/agent-toolkit roles or profiles constrained by read-only default posture.
- Unknowns remain gaps until reviewed; they are not silently treated as disposable.

## IAM users

```text
{
    "Users": [
        {
            "Path": "/",
            "UserName": "daniel.austin.admin",
            "UserId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin",
            "CreateDate": "2026-06-20T01:50:50+00:00",
            "PasswordLastUsed": "2026-08-25T17:16:25+00:00"
        }
    ]
}
```

## IAM roles

```text
{
    "Roles": [
        {
            "Path": "/",
            "RoleName": "adl-codefriend-codebuild-service-role",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/adl-codefriend-codebuild-service-role",
            "CreateDate": "2026-07-04T07:00:47+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "codebuild.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "Description": "ADL CodeFriend CodeBuild service role",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "adl-codefriend-github-actions-build-role",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/adl-codefriend-github-actions-build-role",
            "CreateDate": "2026-07-04T07:00:49+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Federated": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:oidc-provider/token.actions.githubusercontent.com"
                        },
                        "Action": "sts:AssumeRoleWithWebIdentity",
                        "Condition": {
                            "StringEquals": {
                                "token.actions.githubusercontent.com:aud": "sts.amazonaws.com"
                            },
                            "StringLike": {
                                "token.actions.githubusercontent.com:sub": [
                                    "repo:danielbaustin/agent-design-language:ref:refs/heads/main",
                                    "repo:danielbaustin/agent-design-language:ref:refs/heads/codex/*"
                                ]
                            }
                        }
                    }
                ]
            },
            "Description": "ADL CodeFriend GitHub Actions CodeBuild start role",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "adl-csm-5039-api-gateway-bridge-role",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/adl-csm-5039-api-gateway-bridge-role",
            "CreateDate": "2026-07-10T00:33:24+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "lambda.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "adl-csm-notice-receiver-4998-role",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/adl-csm-notice-receiver-4998-role",
            "CreateDate": "2026-07-07T01:50:18+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "lambda.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "adl-ebs-unattached-age-alert-role",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/adl-ebs-unattached-age-alert-role",
            "CreateDate": "2026-08-25T16:27:01+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "lambda.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "Description": "",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "adl-ec2-instance-age-alert-role",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/adl-ec2-instance-age-alert-role",
            "CreateDate": "2026-07-07T18:56:39+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "lambda.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "adl-spot-remote-validation-github-actions-role",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/adl-spot-remote-validation-github-actions-role",
            "CreateDate": "2026-07-04T08:25:07+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Federated": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:oidc-provider/token.actions.githubusercontent.com"
                        },
                        "Action": "sts:AssumeRoleWithWebIdentity",
                        "Condition": {
                            "StringEquals": {
                                "token.actions.githubusercontent.com:aud": "sts.amazonaws.com"
                            },
                            "StringLike": {
                                "token.actions.githubusercontent.com:sub": [
                                    "repo:danielbaustin/agent-design-language:ref:refs/heads/main",
                                    "repo:danielbaustin/agent-design-language:ref:refs/heads/codex/*",
                                    "repo:danielbaustin/agent-design-language:environment:adl-spot-ci"
                                ]
                            }
                        }
                    }
                ]
            },
            "Description": "ADL Spot remote validation GitHub Actions role",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260819-22",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260819-22",
            "CreateDate": "2026-08-19T21:05:39+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260819-27",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260819-27",
            "CreateDate": "2026-08-19T22:19:33+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260820-38",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-268-issue268-six-hour-r7i-20260820-38",
            "CreateDate": "2026-08-21T01:20:18+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]42",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]42",
            "CreateDate": "2026-06-30T18:32:08+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]43",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]43",
            "CreateDate": "2026-06-30T19:33:11+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]41",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]41",
            "CreateDate": "2026-07-01T16:49:43+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]08",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]08",
            "CreateDate": "2026-07-01T18:23:47+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]07",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]07",
            "CreateDate": "2026-07-01T19:44:17+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]30",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]30",
            "CreateDate": "2026-07-01T21:49:54+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]53",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]53",
            "CreateDate": "2026-07-01T21:59:56+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]19",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4603-adl-aws-remote-[AWS_ACCOUNT_ID_REDACTED]19",
            "CreateDate": "2026-07-01T22:09:29+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4837-adl-wp-4837-aws-spot-20260707013",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4837-adl-wp-4837-aws-spot-20260707013",
            "CreateDate": "2026-07-07T01:32:58+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4879-spot-fixed-builder-image-ebs-202",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4879-spot-fixed-builder-image-ebs-202",
            "CreateDate": "2026-07-05T06:37:51+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4879-spot-fixed-builder-image-ebs-war",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4879-spot-fixed-builder-image-ebs-war",
            "CreateDate": "2026-07-05T06:48:09+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4919-adl-wp07-4919-spot-20260706",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4919-adl-wp07-4919-spot-20260706",
            "CreateDate": "2026-07-06T11:15:25+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-4921-adl-wp07-4921-spot-20260706",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-4921-adl-wp07-4921-spot-20260706",
            "CreateDate": "2026-07-06T12:41:29+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5211-issue-5211-systemd-containment",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5211-issue-5211-systemd-containment",
            "CreateDate": "2026-07-12T03:45:59+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29191264648-1",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29191264648-1",
            "CreateDate": "2026-07-12T11:44:34+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29321998194-1-ad",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29321998194-1-ad",
            "CreateDate": "2026-07-14T09:32:13+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29322484555-1-ad",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5243-adl-wp-5243-gha-29322484555-1-ad",
            "CreateDate": "2026-07-14T09:39:52+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-0c3d5e093e",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-0c3d5e093e",
            "CreateDate": "2026-07-27T16:32:39+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-79262133a739",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-79262133a739",
            "CreateDate": "2026-07-27T16:53:03+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-lifecycle-222f",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-lifecycle-222f",
            "CreateDate": "2026-07-28T16:56:31+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-lifecycle-4a38",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5344-adl-wp-5344-linux-lifecycle-4a38",
            "CreateDate": "2026-07-28T17:21:05+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29470867446-1-ad",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29470867446-1-ad",
            "CreateDate": "2026-07-16T04:18:25+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29477416287-1-ad",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29477416287-1-ad",
            "CreateDate": "2026-07-16T06:41:38+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29632596016-1-ad",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5419-adl-wp-5419-gha-29632596016-1-ad",
            "CreateDate": "2026-07-18T05:42:48+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLAwsRemoteValidationRole-5574-adl-wp-5574-gha-29700301599-1-ad",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLAwsRemoteValidationRole-5574-adl-wp-5574-gha-29700301599-1-ad",
            "CreateDate": "2026-07-19T19:22:18+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLHybridSSMManagedNodeRole",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLHybridSSMManagedNodeRole",
            "CreateDate": "2026-06-20T03:06:44+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ssm.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLRemoteValidationPermanentRole",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLRemoteValidationPermanentRole",
            "CreateDate": "2026-08-08T02:51:28+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "ADLWP5795GpuDeadlineReaperRole",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/ADLWP5795GpuDeadlineReaperRole",
            "CreateDate": "2026-08-08T21:09:50+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "lambda.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "Description": "Terminate overdue issue 5795 GPU proof instances",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "agent-logic-ai-github-deploy",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/agent-logic-ai-github-deploy",
            "CreateDate": "2026-07-03T22:47:13+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Sid": "GitHubActionsAssumeRole",
                        "Effect": "Allow",
                        "Principal": {
                            "Federated": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:oidc-provider/token.actions.githubusercontent.com"
                        },
                        "Action": "sts:AssumeRoleWithWebIdentity",
                        "Condition": {
                            "StringEquals": {
                                "token.actions.githubusercontent.com:aud": "sts.amazonaws.com",
                                "token.actions.githubusercontent.com:sub": "repo:agent-logic/agent-logic.ai:ref:refs/heads/main"
                            }
                        }
                    }
                ]
            },
            "Description": "GitHub Actions deploy role for the Agent Logic website",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/aws-service-role/ops.apigateway.amazonaws.com/",
            "RoleName": "AWSServiceRoleForAPIGateway",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/ops.apigateway.amazonaws.com/AWSServiceRoleForAPIGateway",
            "CreateDate": "2026-07-10T00:34:10+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ops.apigateway.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "Description": "The Service Linked Role is used by Amazon API Gateway.",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/aws-service-role/spot.amazonaws.com/",
            "RoleName": "AWSServiceRoleForEC2Spot",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/spot.amazonaws.com/AWSServiceRoleForEC2Spot",
            "CreateDate": "2026-06-27T08:15:55+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "spot.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "Description": "Default EC2 Spot Service Linked Role",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/aws-service-role/elasticloadbalancing.amazonaws.com/",
            "RoleName": "AWSServiceRoleForElasticLoadBalancing",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/elasticloadbalancing.amazonaws.com/AWSServiceRoleForElasticLoadBalancing",
            "CreateDate": "2026-08-26T18:00:34+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "elasticloadbalancing.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "Description": "Allows ELB to call AWS services on your behalf.",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/aws-service-role/resource-explorer-2.amazonaws.com/",
            "RoleName": "AWSServiceRoleForResourceExplorer",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/resource-explorer-2.amazonaws.com/AWSServiceRoleForResourceExplorer",
            "CreateDate": "2026-06-20T01:31:30+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "resource-explorer-2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/aws-service-role/servicequotas.amazonaws.com/",
            "RoleName": "AWSServiceRoleForServiceQuotas",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/servicequotas.amazonaws.com/AWSServiceRoleForServiceQuotas",
            "CreateDate": "2026-07-04T08:01:06+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "servicequotas.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "Description": "A service-linked role is required for Service Quotas to access your service limits.",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/aws-service-role/support.amazonaws.com/",
            "RoleName": "AWSServiceRoleForSupport",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/support.amazonaws.com/AWSServiceRoleForSupport",
            "CreateDate": "2026-06-20T00:56:17+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "support.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "Description": "Enables resource access for AWS to provide billing, administrative and support services",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/aws-service-role/trustedadvisor.amazonaws.com/",
            "RoleName": "AWSServiceRoleForTrustedAdvisor",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/aws-service-role/trustedadvisor.amazonaws.com/AWSServiceRoleForTrustedAdvisor",
            "CreateDate": "2026-06-20T00:56:17+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "trustedadvisor.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "Description": "Access for the AWS Trusted Advisor Service to help reduce cost, increase performance, and improve security of your AWS environment.",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "codefriend-ai-github-deploy",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/codefriend-ai-github-deploy",
            "CreateDate": "2026-07-03T21:20:59+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Sid": "GitHubActionsAssumeRole",
                        "Effect": "Allow",
                        "Principal": {
                            "Federated": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:oidc-provider/token.actions.githubusercontent.com"
                        },
                        "Action": "sts:AssumeRoleWithWebIdentity",
                        "Condition": {
                            "StringEquals": {
                                "token.actions.githubusercontent.com:aud": "sts.amazonaws.com",
                                "token.actions.githubusercontent.com:sub": "repo:agent-logic/codefriend.ai:ref:refs/heads/main"
                            }
                        }
                    }
                ]
            },
            "Description": "GitHub Actions deploy role for the CodeFriend website",
            "MaxSessionDuration": 3600
        },
        {
            "Path": "/",
            "RoleName": "scr-archive-relay-role",
            "RoleId": "[AWS_IDENTIFIER_REDACTED]",
            "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:role/scr-archive-relay-role",
            "CreateDate": "2026-06-30T18:52:08+00:00",
            "AssumeRolePolicyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                    {
                        "Effect": "Allow",
                        "Principal": {
                            "Service": "ec2.amazonaws.com"
                        },
                        "Action": "sts:AssumeRole"
                    }
                ]
            },
            "MaxSessionDuration": 3600
        }
    ]
}
```

## IAM groups

```text
{
    "Groups": []
}
```

