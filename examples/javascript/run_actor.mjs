// JavaScript / TypeScript - official apify-client
import { ApifyClient } from 'apify-client';

const client = new ApifyClient({ token: process.env.APIFY_TOKEN });
const run = await client.actor('data_foundry/aso-app-rank-tracker').call({
    "apps": [
        "389801252",
        {
            "id": "310633997",
            "country": "gb"
        }
    ],
    "trackCharts": true
});
console.log('dataset:', run.defaultDatasetId);

const { items } = await client.dataset(run.defaultDatasetId).listItems();
items.forEach((item) => console.log(item));
