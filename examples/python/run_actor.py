"""Python - official apify-client."""
import os
from apify_client import ApifyClient

client = ApifyClient(os.environ["APIFY_TOKEN"])
run = client.actor("data_foundry/aso-app-rank-tracker").call(run_input={
    "apps": [
        "389801252",
        {
            "id": "310633997",
            "country": "gb"
        }
    ],
    "trackCharts": true
})
print("dataset:", run["defaultDatasetId"])
for item in client.dataset(run["defaultDatasetId"]).iterate_items():
    print(item)
