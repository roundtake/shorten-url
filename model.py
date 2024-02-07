from motor.motor_asyncio import AsyncIOMotorClient
import asyncio

class Model:
    # 🐛😱 buuuuuuuugs!
    collection = None

    def __init__(self):
        client = AsyncIOMotorClient("mongodb://localhost:27017")
        db = client.roundtake
        self.collection = db.shorten_url
        self.loop = client.get_io_loop()

    @classmethod
    async def insert(cls, document: dict):
        result = await cls.collection.insert_one(document)
        return result
    
    @classmethod
    async def find(cls, query: dict):
        result = await cls.collection.find_one(query)
        # return cursor.to_list(length=1)
        return result