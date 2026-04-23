from fastapi import FastAPI
from pydantic import BaseModel


app = FastAPI()

sales = []

class Venda(BaseModel):
    id: int
    produto: str
    quantidade: int
    preco: float

@app.post('/vendas')
def fn_sale(venda: Venda):
    sales.append(venda)
    return {'message': 'Venda concluida'}

@app.get('/vendas')
def sale_list():
    return {'vendas': sales}

@app.get('/vendas/{venda_id}')
def search_sale(venda_id: int):
    for v in sales:
        if v.id == venda_id:
            return {'venda': v}
    return {'error': 'Venda não encontrada'}

@app.get('/total')
def total_sales():
    total = sum(v.quantidade * v.preco for v in sales)
    return {'total': total}