def count_possible_xor_values(A):
    basis = []  # XOR基底
    for x in A:
        for b in basis:
            x = min(x, x ^ b)  # 基底とXORして簡約化
        if x > 0:
            basis.append(x)
            basis.sort(reverse=True)  # 基底を大きい順にソート

    # 基底を用いて構築可能な値の数
    return 2 ** len(basis)

# 入力
N = int(input())
A = list(map(int, input().split()))

# 計算
result = count_possible_xor_values(A)
print(result)
