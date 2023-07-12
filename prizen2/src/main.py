import numpy as np
import typing

class Layer: 
    weights: np.ndarray
    biases: np.ndarray
    func: typing.Callable[[np.ndarray], np.ndarray]

    def __init__(self, factor: int, prev_size: int, size: int, func):
        self.weights = factor * np.random.rand(size, prev_size)
        self.biases = factor * np.random.rand(size)
        self.func = func

    def propagate_forward(self, a: np.ndarray) -> np.ndarray:
        return self.func(self.weights.dot(a) + self.biases)

class Network:
    layers: list[Layer]

    def __init__(self, sizes: list[int], funcs, factor: int=1):
        layers = []
        for (i, el) in enumerate(sizes[:-1]):
            current_size = el
            next_size = sizes[i+1]
            layers.append(Layer(factor, current_size, next_size, funcs[i]))
        self.layers = layers

    def model(self, a: np.ndarray):
        for layer in self.layers:
            a = layer.propagate_forward(a)
        return a

a = Network([3, 7, 1], funcs=[lambda x: x, lambda x: x]).model(np.array([3, 4, 5]))
print(a)