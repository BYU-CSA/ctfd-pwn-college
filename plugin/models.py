from CTFd.models import db

class PrettyPrinted(db.Model):
    __tablename__ = "pretty_printed_pwn_college"

    name = db.Column(db.String(128), primary_key=True)
    pretty_name = db.Column(db.Text, nullable=False)
