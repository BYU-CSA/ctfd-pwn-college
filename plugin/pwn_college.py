import requests
from dataclasses import dataclass
from CTFd.models import db
from .models import PrettyPrinted

URL = "https://pwn.college"
dojo = "intro-to-cybersecurity"

@dataclass
class Solve:
    def __init__(self, timestamp, module_id, challenge_id):
        self.timestamp = timestamp
        self.module_id = module_id
        self.challenge_id = challenge_id
    
    @classmethod
    def from_ctfd_api(cls, api_object):
        return cls(
            timestamp=None,
            module_id=None,
            challenge_id=api_object.challenge.name
        )

    @classmethod
    def from_pwn_college_api(cls, api_object):
        return cls(
            timestamp=api_object["timestamp"],
            module_id=api_object["module_id"],
            challenge_id=challenge_to_pretty(api_object["challenge_id"])
        )

    def __eq__(self, other):
        if not isinstance(other, Solve):
            return NotImplemented
        return self.challenge_id == other.challenge_id

    def __str__(self):
        if self.module_id != None:
            return f"Solve for {self.challenge_id} in {self.module_id}"
        else:
            return f"Solve for {self.challenge_id}"
    
    def __repr__(self):
        return f"Solve(timestamp={self.timestamp}, module_id={self.module_id}, challenge_id={self.challenge_id})"
    
    def __hash__(self):
        return hash(self.challenge_id)


def get_solves_by_user_for_dojo(
    username: str,
) -> list[Solve]:
    url = f"{URL}/pwncollege_api/v1/dojos/{dojo}/solves?username={username}"
    response = requests.get(url).json()

    if response["success"]:
        # edited this to support pretty print
        return [Solve.from_pwn_college_api(d) for d in response["solves"]]

    return []

def challenge_to_pretty(name: str) -> str:
    pretty_name = PrettyPrinted.query.get(name)
    if pretty_name is None:
        url = f"{URL}/pwncollege_api/v1/dojos/{dojo}/modules"
        response = requests.get(url).json()

        if response["success"]:
            all_data = response["modules"]
            for module in all_data:
                module_challs = module["challenges"]
                for chall in module_challs:
                    if chall["id"] == name:
                        entry = PrettyPrinted(name=name, pretty_name=chall["name"])
                        db.session.add(entry)
                        db.session.commit()
                        return chall["name"]

        return ""
    else:
        return pretty_name.pretty_name