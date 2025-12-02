from functools import wraps
from typing import Callable, Any


def externa(func: Callable) -> Callable:
    @wraps(func)  # aqui func é uma variavel livre
    def interna(*args, **kwargs):
        print(f'Estou decorando a função: {func.__name__}')
        result = func(*args, **kwargs)
        print('Terminei de decorar a função.')
        return result
    return interna


@externa
def soma(x: int, y: int) -> int:
    return x + y

a = soma(2, 2)
print(a)
print(soma.__code__.co_freevars)
for cell in soma.__closure__:
    print(cell.cell_contents)


@lambda fn: fn()
def faz_qlqr_coisa():
    print('Essa função faz qlqr coisa')

faz_qlqr_coisa


def fabrica_decorators(n_times: int = 3):
    def externa(func: Callable) -> Callable:
        @wraps(func)
        def interna(*args, **kwargs):
            print(f'Eu vou fazer essa função rodar {n_times} vezes')
            return [func(*args, **kwargs) for _ in range(n_times)]
        return interna
    return externa

@fabrica_decorators(n_times=5)
def soma(x: int, y: int) -> int:
    return x + y

varias_somas = soma(2, 7)
print(varias_somas)


