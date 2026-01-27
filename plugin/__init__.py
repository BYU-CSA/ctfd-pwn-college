from flask import Blueprint
from .sync import sync_blueprint
from CTFd.models import db
from .models import PrettyPrinted

def load(app) -> None:
    """
    This function is called by CTFd when the plugin is loaded.
    You register blueprints, hooks, etc. here.
    """
    with app.app_context():
        # On change, this needs to be added in to reset the table
        # PrettyPrinted.__table__.drop(db.engine)
        db.create_all()

    # Register our blueprint (routes)
    app.register_blueprint(sync_blueprint)

    # You can also register signals, CLI commands, etc. here later
