from fastapi import FastAPI, HTTPException
from fastapi.responses import RedirectResponse
import uvicorn
import random
import model

app = FastAPI(root_path='/api/v1/shorten-url')


# @app.get('/{id}')
# async def redirect(id):
#     result = await model.find({'shorten_url': id})
#     if result == None:
#         raise HTTPException(status_code=404)
#     return RedirectResponse(f'{result["full_url"]}')

@app.get('/{id}')
async def get_full_url_code(id):
    result = await model.find({'shorten_url': id})
    if result == None:
        raise HTTPException(status_code=404)
    return {'full_url': result["full_url"]}

@app.post('/')
async def add_shorten_url_code(full_url: str, shorten_url: str | None = None):
    # if shorten_url is specified
    if shorten_url != None:
        result = await model.find({'shorten_url': shorten_url})
        if result != None:
            return {'shorten_url_code': result['shorten_url']}
        else:
            await model.insert({
                'shorten_url': shorten_url, 
                'full_url': full_url
            })
            # return the shorten url code
            return {'shorten_url_code': shorten_url}
    # else: if shorten_url is not specified
    # search for full_url 
    result = await model.find({'full_url': full_url})
    # full_url exist
    if result != None:
        return {'shorten_url_code': result['shorten_url']}
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
        result = await model.find({'shorten_url': shorten_url})
        if result == None:  # no repeat
            repeat = False
    # add shorten_url to database
    await model.insert({
        'shorten_url': shorten_url, 
        'full_url': full_url
    })
    # return the shorten url code
    return {'shorten_url_code': shorten_url}

if __name__ == '__main__':
    uvicorn.run('main:app', host='127.0.0.1', port=8006, reload=True)