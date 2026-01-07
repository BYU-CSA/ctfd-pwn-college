import requests
import os
import secrets
import sys

PWN_URL = "https://pwn.college"
CTFD_URL = os.environ.get("CTFD_URL")
CTFD_API_KEY = os.environ.get("CTFD_API_KEY")

def get_challenges_for_module(module):
    url = f"{PWN_URL}/pwncollege_api/v1/dojos/intro-to-cybersecurity/modules"
    response = requests.get(url).json()["modules"]

    target_module = [d for d in response if d["id"] == module][0]

    return {
        "module-id": target_module["id"],
        "module-name": target_module["name"],
        "challenges": target_module["challenges"]
    }

def new_ctfd_flag_for_challenge(id, flag: str,):
    new_flag = {
        "content": flag,
        "data": "case_insensitive",
        "type": "static",
        "challenge": id,
    }
    headers = {
        "Authorization": f"Token {CTFD_API_KEY}",
        "Content-Type": "application/json",
    }

    url = f"{CTFD_URL}/api/v1/flags"
    response = requests.post(url, headers=headers, json=new_flag)

    if not response.ok:
        raise Exception("Get request failed")

def get_ctfd_challenge_id_by_name(name: str,):
    url = f"{CTFD_URL}/api/v1/challenges?name={name}"
    headers = {
        "Authorization": f"Token {CTFD_API_KEY}",
        "Content-Type": "application/json",
    }
    
    response = requests.get(url, headers=headers).json()["data"]

    if len(response) > 0:
        return response[0]
    return None

def new_ctfd_challenge(name: str, category: str, flag: str,):
    id = get_ctfd_challenge_id_by_name(name)
    if id:
        print(f"Challenge {name} already has id {id}")
        return id
        
    url = f"{CTFD_URL}/api/v1/challenges"
    new_challenge = {
        "name": name,
        "category": category,
        "description": f"This challenge will be on PWN College in the {category} module and will auto-complete here when you solve it there",
        "value": 100,
        "state": "visible",
        "type": "standard",
    }
    headers = {
        "Authorization": f"Token {CTFD_API_KEY}",
        "Content-Type": "application/json",
    }


    # send the request to make the challenge
    response = requests.post(url, json=new_challenge, headers=headers)

    if not response.ok:
        print(response.text)
        raise Exception("Failed post")

    new_ctfd_flag_for_challenge(id, flag)

    return id

def import_challenges_from_module(module):
    challenges = get_challenges_for_module(module)

    module_name = challenges["module-name"]

    flag = secrets.token_bytes(32).hex()

    for challenge in challenges["challenges"]:
        id = new_ctfd_challenge(challenge["name"], module_name, flag)
        print(f"Added new challenge: {challenge["name"]}")
    
    with open('./flags.txt', 'a') as f:
        f.write(f'\n{module}: {flag}')
    with open('./plugin/modules.txt', 'a') as f:
        f.write(f'\n{module}')


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Please run the script with an argument (the module to be added)")
        quit()
    module_name = sys.argv[1]
    import_challenges_from_module(module_name)