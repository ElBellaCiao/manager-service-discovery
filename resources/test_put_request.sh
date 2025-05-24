#!/bin/bash
curl -X PUT https://k1vptg40ha.execute-api.us-east-1.amazonaws.com/service-discovery/i-0c2b5116e3d789615 \
  -H "Content-Type: application/json" \
  -d @resources/service_discovery_put_request.json
