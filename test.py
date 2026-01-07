import os
import requests
from pwn_college import *

def test_ctfd():
    # Read environment variables
    CTFD_URL = os.environ.get("CTFD_URL")  # e.g., "https://ctfd.example.com"
    CTFD_API_KEY = os.environ.get("CTFD_API_KEY")  # admin API key

    if not CTFD_URL or not CTFD_API_KEY:
        raise ValueError("CTFD_URL and CTFD_API_KEY must be set in environment variables")

    # Full plugin endpoint
    sync_endpoint = f"{CTFD_URL}/plugins/external_solve_sync/sync"

    # Headers: admin token
    headers = {
        "Authorization": f"Token {CTFD_API_KEY}",
        "Content-Type": "application/json",
    }

    # POST request (no body needed for our example plugin)
    response = requests.post(sync_endpoint, headers=headers)

    # Check response
    if response.ok:
        data = response.json()
        print(f"Sync completed. {data.get('created_solves', 0)} new solves created.")
    else:
        print(f"Sync failed: {response.status_code} {response.text}")

def test_pwn_college():
    print(get_solves_by_user_for_dojo("overllama"))

if __name__ == "__main__":
    # test_pwn_college()
    test_ctfd()