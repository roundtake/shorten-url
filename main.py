from fastapi import FastAPI
from fastapi.responses import RedirectResponse
import uvicorn
import random
from model import Model

app = FastAPI()


@app.get('/{id}')
async def redirect(id):
    result = await Model.find({'shorten_url': id})
    return RedirectResponse(f'/{result[0]}')

@app.post('/shorten-url')
async def add_shorten_url(full_url: str):
    # search for full_url 
    result = await Model.find({'full_url': full_url})
    # full_url exist
    if len(result) > 0:
        return result.shorten_url
    # full_url not exist
    # all characters can be used in shorten_url code
    characters = ('0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
                  'a', 'b', 'c', 'd', 'f', 'g', 'h', 'i', 'j', 'k',  # no 'e'(pronounce like '1' in Chinese)
                  'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',  # no 'l'(like symbol 'I'), 'o'(like symbol '0')
                  'x', 'y', 'z')
    # random choose 6 characters until not repeat
    repeat = True
    while repeat:
        # generate shorten url code
        shorten_url = ''
        for i in range(6):
            shorten_url += random.choice(characters)
        # find exist repeat shorten url code
        result = await Model.find({'shorten_url': shorten_url})
        if result == None or len(result) == 0:  # no repeat
            repeat = False
    # add shorten_url to database
    await Model.insert({
        'shorten_url': shorten_url, 
        'full_url': full_url
    })
    # return the shorten url code
    return {'shorten_url': f'/{shorten_url}'}

if __name__ == '__main__':
    uvicorn.run('main:app', host='127.0.0.1', port=8003, reload=True)