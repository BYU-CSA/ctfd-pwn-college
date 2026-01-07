import os, requests

def sync():
    CTFD_URL = os.environ.get("CTFD_URL")
    CTFD_API_KEY = os.environ.get("CTFD_API_KEY")

    if not CTFD_URL or not CTFD_API_KEY:
        raise ValueError("CTFD_URL and CTFD_API_KEY must be set in environment variables")

    sync_endpoint = f"{CTFD_URL}/plugins/external_solve_sync/sync"

    # make sure you have a valid admin token
    headers = {
        "Authorization": f"Token {CTFD_API_KEY}",
        "Content-Type": "application/json",
    }

    response = requests.post(sync_endpoint, headers=headers)

    if response.ok:
        data = response.json()
        if data.get('created_solves', 0) > 0:
            print(f"Sync completed. {data.get('created_solves', 0)} new solves created.")
    else:
        print(f"Sync failed: {response.status_code} {response.text}")

sync()
