#!/bin/bash
curl -X PUT https://k1vptg40ha.execute-api.us-east-1.amazonaws.com/service-discoverability/i-03bfa510b332963dd \
  -H "Content-Type: application/json" \
  -d @resources/service_discoverability_put_request.json
