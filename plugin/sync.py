from flask import Blueprint, jsonify, Response
from CTFd.utils.decorators import admins_only
from CTFd.models import db, Users, Challenges, Solves
from datetime import datetime
from .pwn_college import *
import os

sync_blueprint = Blueprint(
    "external_solve_sync",
    __name__,
    url_prefix="/plugins/external_solve_sync",
)

# https://github.com/CTFd/CTFd/blob/master/CTFd/models/__init__.py#L392
def has_pwn_college_username(user) -> bool:
    fields = user.get_fields()
    
    if len(fields) > 0:
        for entry in fields:
            if entry.field.name == 'PWN College Username':
                return True
            else:
                print(entry.field.name)

    return False

def get_pwn_college_username(user) -> str:
    for entry in user.get_fields():
        if entry.field.name == 'PWN College Username':
            return entry.value
    raise ValueError("No valid key, did the filter not work?")


@sync_blueprint.route("/sync", methods=["POST"])
@admins_only
def sync_solves() -> Response:
    """
    Admin-only endpoint to sync solves from an external system.

    You can trigger this manually or via a cron job / webhook.

    Each tick should:
        - for each user
            - gather old and new solves and find unique new solves
            - submit those for the user on CTFd
    """

    # get users first
    users = Users.query.all()

    # do a filter() here with has_pwn_college_username()
    valid_users = filter(has_pwn_college_username, users)

    created = 0
    for user in valid_users:
        created += resolve_external_solves(user)

    return jsonify({
        "success": True,
        "created_solves": created,
    }) 

def get_current_modules() -> list[str]:
    dir_path = os.path.dirname(os.path.realpath(__file__))
    module_file = os.path.join(dir_path, 'modules.txt')

    with open(module_file, 'r') as f:
        # SafeLoad prevents execution of arbitrary code in the YAML file
        return f.read().splitlines()

def resolve_external_solves(user: Users) -> int:
    username = get_pwn_college_username(user)
    queried_solves = Solves.query.filter_by(id=user.account_id).all()
    old_solves = [Solve.from_ctfd_api(d) for d in queried_solves]

    modules = get_current_modules()
    
    new_solves = list(filter(lambda x: x not in old_solves and x.module_id in modules, get_solves_by_user_for_dojo(username)))

    created = 0
    for solve in new_solves:
        challenge = Challenges.query.filter_by(name=challenge_to_pretty(solve.challenge_id)).first()
        if challenge:
            db.session.add(Solves(
                user_id = user.id,
                challenge_id = challenge.id,
                date = datetime.utcnow(),
                ip="127.0.0.1"
            ))

            created += 1
    
    db.session.commit()

    return created