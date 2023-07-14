import numpy as np

def default(x: float) -> float: return x
def sigmoid(x: float) -> float: return 1/(1+np.exp(-x))