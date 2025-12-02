A = [[2, 2], [1, 1]]
B = [[1, 1], [4, 1]]


type Matrix = list[list[float], ...]


# A . B = [[10, 4], [5, 2]]
# B . A = [[3, 3], [9, 9]]


def matrix_mul(A: Matrix, B: Matrix) -> Matrix:
    C = [[0, 0], [0, 0]]
    n_rows = len(A)
    n_cols = len(B[0])
    n = len(B)
    for i in range(n_rows):
        for j in range(n_cols):
            for k in range(n):
                C[i][j] += A[i][k] * B[k][j]
    return C

print(matrix_mul(A, B))
print(matrix_mul(B, A))