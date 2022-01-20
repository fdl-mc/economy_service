import ormar

from economy_service.api.database import database, metadata


class EconomyStates(ormar.Model):
    class Meta:
        tablename: str = "economy_states"
        database = database
        metadata = metadata

    id: int = ormar.Integer(primary_key=True)
    balance: int = ormar.Integer(default=0)
