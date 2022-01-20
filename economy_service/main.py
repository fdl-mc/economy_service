from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from economy_service.api.database import database, engine, metadata
from economy_service.api.routes.economy import economy_router
from economy_service.api.routes.transactions import transactions_rotuer

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
app.include_router(economy_router)
app.include_router(transactions_rotuer)
metadata.create_all(engine)


@app.on_event("startup")
async def startup() -> None:
    if not database.is_connected:
        await database.connect()


@app.on_event("shutdown")
async def shutdown() -> None:
    if database.is_connected:
        await database.disconnect()
