# CSM Runtime Spot host

This root stack creates one disposable Spot EC2 instance for Runtime testing.
It is deliberately separate from the ALB and public edge stacks so the instance
can be killed and recreated quickly.

The stack does not bake in Runtime secrets. Use `user_data` or an operator-run
bootstrap after launch, and keep secret values in local ignored files or AWS
secret stores.

Fast path:

1. Create the ALB first, or decide on a direct operator smoke CIDR.
2. Apply this stack with `alb_security_group_id` set to the ALB output, or with
   `operator_ingress_cidrs` set to a narrow `/32` for direct smoke testing.
3. Start Runtime on `0.0.0.0:20997` with a certificate matching the ALB origin
   hostname.
4. Attach the instance id to the ALB stack.

For smoke-only proof, `user_data_file` can point at a local ignored bootstrap
script that starts a tiny HTTPS health responder. The #122 live smoke used that
path to prove the external ALB call reached the EC2 instance, then destroyed all
Spot resources.
