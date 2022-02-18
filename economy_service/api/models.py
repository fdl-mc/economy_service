import ormar

from economy_service.api.database import database, metadata


class EconomyStates(ormar.Model):
    class Meta:
        tablename: str = "economy_states"
        database = database
        metadata = metadata

    id: int = ormar.Integer(primary_key=True)
    balance: int = ormar.Integer(default=0)


class Transaction(ormar.Model):
    class Meta:
        tablename: str = "transactions"
        database = database
        metadata = metadata

    id: int = ormar.Integer(primary_key=True, autoincrement=True)
    payer_id: int = ormar.Integer()
    payee_id: int = ormar.Integer()
    amount: int = ormar.Integer()
    comment: str = ormar.String(max_length=512, nullable=True)
