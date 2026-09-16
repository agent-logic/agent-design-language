+++
status = "accepted"
boundary = "core"
service = "api"
decision_key = "api-deployment"
choice = "separate-service"
+++
# API deployment

Keep the API in a separate service to permit independent rollout of its image.
This records the human decision; it does not claim runtime rollout was tested.
