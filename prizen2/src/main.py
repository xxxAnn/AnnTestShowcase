import numpy as np
import typing
import math

class Layer: 
    weights: np.ndarray
    biases: np.ndarray
    func: typing.Callable[[float], float]

    def __init__(self, factor: int, prev_size: int, size: int, func):
        self.weights = factor * np.random.rand(size, prev_size)
        self.biases = factor * np.random.rand(size)
        self.func = np.vectorize(func)

    def propagate_forward(self, a: np.ndarray) -> np.ndarray:
        res = self.func(self.weights.dot(a) + self.biases)
        #//print(f"Propagating: {res}")
        return res

class Network:
    layers: list[Layer]

    def __init__(self, sizes: list[int], funcs, factor: int=1):
        layers = []
        for (i, el) in enumerate(sizes[:-1]):
            current_size = el
            next_size = sizes[i+1]
            layers.append(Layer(factor, current_size, next_size, get(funcs, i)))
        self.layers = layers

    def model(self, a: np.ndarray):
        for layer in self.layers:
            a = layer.propagate_forward(a)
        return a
    
def get(funcs, i, default=None):
    if i < len(funcs):
        res = funcs[i]
    else:
        res = default

    if res is None: res = lambda x: x

    return res
    
    
def sigmoid(x: float) -> float:
    return 1/(1+np.exp(-x))

n = Network([3, 12, 8, 3], funcs=[sigmoid] * 4)
a = n.model(np.array([-100, -100, -100]))
print(a)