from flask import Blueprint
from .sync import sync_blueprint

def load(app) -> None:
    """
    This function is called by CTFd when the plugin is loaded.
    You register blueprints, hooks, etc. here.
    """

    # Register our blueprint (routes)
    app.register_blueprint(sync_blueprint)

    # You can also register signals, CLI commands, etc. here later
