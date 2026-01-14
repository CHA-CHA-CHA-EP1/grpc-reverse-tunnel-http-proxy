curl -k -X POST --location 'http://127.0.0.1:8080/api/message' \
  -H 'Content-Type: application/json' \
  --data '{
    "agent": "dgl-gcp-uat",
    "message": "aws-gcp-uat-clear-dynamic-reject 1234"
  }'
